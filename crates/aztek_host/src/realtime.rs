//! The Realtime viewport's shadow map: one depth image drawn from the first light that casts shadows, and the meshes it
//! needs. The shading itself is in `render.rs`; this is what the shader reads.

use std::collections::HashMap;
use std::sync::Arc;

use ash::vk;
use aztek_geo::render_mesh::{build_render_mesh, RenderMeshOptions};
use aztek_geo::Mesh;
use aztek_scene::plug::NodeId;
use glam::DMat4;
use gpu_allocator::vulkan::{Allocation, AllocationCreateDesc, AllocationScheme};
use gpu_allocator::MemoryLocation;

use crate::gpu::{Buffer, Gpu, DEPTH_FORMAT};
use crate::lighting::{ShadowCamera, ShadowPlan};
use crate::render::compile;
use crate::viewport::DrawItem;

/// The shadow map's side: 2048 texels. Enough for one light over a whole scene at a viewport's size.
pub const SHADOW_SIZE: u32 = 2048;

pub(crate) const SHADOW_VS: &str = r#"#version 450
layout(location = 0) in vec3 a_pos;
layout(push_constant) uniform PC { mat4 mvp; } pc;
void main() {
    gl_Position = pc.mvp * vec4(a_pos, 1.0);
}
"#;

pub(crate) const SHADOW_FS: &str = r#"#version 450
void main() {
}
"#;

/// What one shape's shadow-casting triangles are on the GPU.
struct CastMesh {
    source: Arc<Mesh>,
    vb: Buffer,
    ib: Buffer,
    count: u32,
}

pub struct Realtime {
    pass: vk::RenderPass,
    framebuffer: vk::Framebuffer,
    image: vk::Image,
    view: vk::ImageView,
    alloc: Option<Allocation>,
    /// The six faces of a point light's map, as one 2D array the shader samples.
    cube_image: vk::Image,
    cube_view: vk::ImageView,
    cube_alloc: Option<Allocation>,
    cube_faces: [vk::ImageView; 6],
    cube_framebuffers: [vk::Framebuffer; 6],
    layout: vk::PipelineLayout,
    pipeline: vk::Pipeline,
    sampler: vk::Sampler,
    pool: vk::DescriptorPool,
    set: vk::DescriptorSet,
    meshes: HashMap<NodeId, CastMesh>,
}

/// A depth image of the map's size, with one or six layers.
unsafe fn depth_image(gpu: &Gpu, layers: u32, cube: bool) -> (vk::Image, vk::ImageView, Allocation) {
    let d = &gpu.device;
    let mut info = vk::ImageCreateInfo::default()
        .image_type(vk::ImageType::TYPE_2D)
        .format(DEPTH_FORMAT)
        .extent(vk::Extent3D { width: SHADOW_SIZE, height: SHADOW_SIZE, depth: 1 })
        .mip_levels(1)
        .array_layers(layers)
        .samples(vk::SampleCountFlags::TYPE_1)
        .usage(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT | vk::ImageUsageFlags::SAMPLED);
    if cube {
        info = info.flags(vk::ImageCreateFlags::CUBE_COMPATIBLE);
    }
    let image = d.create_image(&info, None).expect("shadow image");
    let req = d.get_image_memory_requirements(image);
    let alloc = gpu
        .shared
        .allocator
        .lock()
        .unwrap()
        .allocate(&AllocationCreateDesc {
            name: "shadow map",
            requirements: req,
            location: MemoryLocation::GpuOnly,
            linear: false,
            allocation_scheme: AllocationScheme::GpuAllocatorManaged,
        })
        .expect("shadow memory");
    d.bind_image_memory(image, alloc.memory(), alloc.offset()).expect("shadow bind");
    let view = d
        .create_image_view(
            &vk::ImageViewCreateInfo::default()
                .image(image)
                .view_type(if cube { vk::ImageViewType::CUBE } else { vk::ImageViewType::TYPE_2D })
                .format(DEPTH_FORMAT)
                .subresource_range(vk::ImageSubresourceRange::default().aspect_mask(vk::ImageAspectFlags::DEPTH).level_count(1).layer_count(layers)),
            None,
        )
        .expect("shadow view");
    (image, view, alloc)
}

impl Realtime {
    pub fn new(gpu: &Gpu, shadow_layout: vk::DescriptorSetLayout) -> Realtime {
        unsafe {
            let d = &gpu.device;
            let (image, view, alloc) = depth_image(gpu, 1, false);
            let (cube_image, cube_view, cube_alloc) = depth_image(gpu, 6, true);
            let mut cube_faces = [vk::ImageView::null(); 6];
            let mut cube_framebuffers = [vk::Framebuffer::null(); 6];
            let attachment = [vk::AttachmentDescription::default()
                .format(DEPTH_FORMAT)
                .samples(vk::SampleCountFlags::TYPE_1)
                .load_op(vk::AttachmentLoadOp::CLEAR)
                .store_op(vk::AttachmentStoreOp::STORE)
                .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
                .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
                .initial_layout(vk::ImageLayout::UNDEFINED)
                .final_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)];
            let depth = vk::AttachmentReference::default().attachment(0).layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL);
            let sub = [vk::SubpassDescription::default().pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS).depth_stencil_attachment(&depth)];
            let dep = [vk::SubpassDependency::default()
                .src_subpass(vk::SUBPASS_EXTERNAL)
                .dst_subpass(0)
                .src_stage_mask(vk::PipelineStageFlags::LATE_FRAGMENT_TESTS | vk::PipelineStageFlags::FRAGMENT_SHADER)
                .dst_stage_mask(vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS)
                .src_access_mask(vk::AccessFlags::SHADER_READ)
                .dst_access_mask(vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE)];
            let pass = d.create_render_pass(&vk::RenderPassCreateInfo::default().attachments(&attachment).subpasses(&sub).dependencies(&dep), None).expect("shadow pass");
            let framebuffer = d
                .create_framebuffer(
                    &vk::FramebufferCreateInfo::default().render_pass(pass).attachments(&[view]).width(SHADOW_SIZE).height(SHADOW_SIZE).layers(1),
                    None,
                )
                .expect("shadow framebuffer");
            for (i, (fv, fb)) in cube_faces.iter_mut().zip(cube_framebuffers.iter_mut()).enumerate() {
                *fv = d
                    .create_image_view(
                        &vk::ImageViewCreateInfo::default()
                            .image(cube_image)
                            .view_type(vk::ImageViewType::TYPE_2D)
                            .format(DEPTH_FORMAT)
                            .subresource_range(
                                vk::ImageSubresourceRange::default()
                                    .aspect_mask(vk::ImageAspectFlags::DEPTH)
                                    .base_array_layer(i as u32)
                                    .layer_count(1)
                                    .level_count(1),
                            ),
                        None,
                    )
                    .expect("shadow face view");
                *fb = d
                    .create_framebuffer(
                        &vk::FramebufferCreateInfo::default().render_pass(pass).attachments(&[*fv]).width(SHADOW_SIZE).height(SHADOW_SIZE).layers(1),
                        None,
                    )
                    .expect("shadow face framebuffer");
            }
            let range = [vk::PushConstantRange::default().stage_flags(vk::ShaderStageFlags::VERTEX).size(64)];
            let layout = d.create_pipeline_layout(&vk::PipelineLayoutCreateInfo::default().push_constant_ranges(&range), None).expect("shadow layout");
            let module = |src: &str, stage| {
                let code = compile(src, stage);
                d.create_shader_module(&vk::ShaderModuleCreateInfo::default().code(&code), None).expect("shadow module")
            };
            let vs = module(SHADOW_VS, naga::ShaderStage::Vertex);
            let fs = module(SHADOW_FS, naga::ShaderStage::Fragment);
            let stages = [
                vk::PipelineShaderStageCreateInfo::default().stage(vk::ShaderStageFlags::VERTEX).module(vs).name(c"main"),
                vk::PipelineShaderStageCreateInfo::default().stage(vk::ShaderStageFlags::FRAGMENT).module(fs).name(c"main"),
            ];
            let binding = [vk::VertexInputBindingDescription::default().stride(12).input_rate(vk::VertexInputRate::VERTEX)];
            let attrs = [vk::VertexInputAttributeDescription::default().location(0).format(vk::Format::R32G32B32_SFLOAT).offset(0)];
            let vin = vk::PipelineVertexInputStateCreateInfo::default().vertex_binding_descriptions(&binding).vertex_attribute_descriptions(&attrs);
            let ia = vk::PipelineInputAssemblyStateCreateInfo::default().topology(vk::PrimitiveTopology::TRIANGLE_LIST);
            let vp = vk::PipelineViewportStateCreateInfo::default().viewport_count(1).scissor_count(1);
            let rs = vk::PipelineRasterizationStateCreateInfo::default()
                .polygon_mode(vk::PolygonMode::FILL)
                .cull_mode(vk::CullModeFlags::NONE)
                .front_face(vk::FrontFace::CLOCKWISE)
                .line_width(1.0)
                .depth_bias_enable(true)
                .depth_bias_constant_factor(1.5)
                .depth_bias_slope_factor(1.5);
            let ms = vk::PipelineMultisampleStateCreateInfo::default().rasterization_samples(vk::SampleCountFlags::TYPE_1);
            let ds = vk::PipelineDepthStencilStateCreateInfo::default()
                .depth_test_enable(true)
                .depth_write_enable(true)
                .depth_compare_op(vk::CompareOp::LESS_OR_EQUAL);
            let cb = vk::PipelineColorBlendStateCreateInfo::default();
            let dyns = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
            let dy = vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dyns);
            let info = vk::GraphicsPipelineCreateInfo::default()
                .stages(&stages)
                .vertex_input_state(&vin)
                .input_assembly_state(&ia)
                .viewport_state(&vp)
                .rasterization_state(&rs)
                .multisample_state(&ms)
                .depth_stencil_state(&ds)
                .color_blend_state(&cb)
                .dynamic_state(&dy)
                .layout(layout)
                .render_pass(pass)
                .subpass(0);
            let pipeline = d.create_graphics_pipelines(vk::PipelineCache::null(), &[info], None).map_err(|e| e.1).expect("shadow pipeline")[0];
            d.destroy_shader_module(vs, None);
            d.destroy_shader_module(fs, None);

            let sizes = [
                vk::DescriptorPoolSize::default().ty(vk::DescriptorType::SAMPLED_IMAGE).descriptor_count(2),
                vk::DescriptorPoolSize::default().ty(vk::DescriptorType::SAMPLER).descriptor_count(2),
            ];
            let pool = d.create_descriptor_pool(&vk::DescriptorPoolCreateInfo::default().max_sets(1).pool_sizes(&sizes), None).expect("shadow pool");
            let set = d.allocate_descriptor_sets(&vk::DescriptorSetAllocateInfo::default().descriptor_pool(pool).set_layouts(&[shadow_layout])).expect("shadow set")[0];
            let sampler = d
                .create_sampler(
                    &vk::SamplerCreateInfo::default()
                        .mag_filter(vk::Filter::NEAREST)
                        .min_filter(vk::Filter::NEAREST)
                        .address_mode_u(vk::SamplerAddressMode::CLAMP_TO_EDGE)
                        .address_mode_v(vk::SamplerAddressMode::CLAMP_TO_EDGE)
                        .address_mode_w(vk::SamplerAddressMode::CLAMP_TO_EDGE),
                    None,
                )
                .expect("shadow sampler");
            let image_info = [vk::DescriptorImageInfo::default().image_view(view).image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)];
            let cube_info = [vk::DescriptorImageInfo::default().image_view(cube_view).image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)];
            let sampler_info = [vk::DescriptorImageInfo::default().sampler(sampler)];
            let writes = [
                vk::WriteDescriptorSet::default().dst_set(set).dst_binding(0).descriptor_type(vk::DescriptorType::SAMPLED_IMAGE).image_info(&image_info),
                vk::WriteDescriptorSet::default().dst_set(set).dst_binding(1).descriptor_type(vk::DescriptorType::SAMPLER).image_info(&sampler_info),
                vk::WriteDescriptorSet::default().dst_set(set).dst_binding(2).descriptor_type(vk::DescriptorType::SAMPLED_IMAGE).image_info(&cube_info),
                vk::WriteDescriptorSet::default().dst_set(set).dst_binding(3).descriptor_type(vk::DescriptorType::SAMPLER).image_info(&sampler_info),
            ];
            d.update_descriptor_sets(&writes, &[]);
            Realtime {
                pass,
                framebuffer,
                image,
                view,
                alloc: Some(alloc),
                cube_image,
                cube_view,
                cube_alloc: Some(cube_alloc),
                cube_faces,
                cube_framebuffers,
                layout,
                pipeline,
                sampler,
                pool,
                set,
                meshes: HashMap::new(),
            }
        }
    }

    /// The shadow map's descriptor set, when it holds this frame's map.
    pub fn set(&self) -> Option<vk::DescriptorSet> {
        Some(self.set)
    }

    /// Draws this frame's shadow map, before the scene's pass. Nothing when the frame plans none.
    pub fn render(&mut self, gpu: &mut Gpu, items: &[DrawItem], plan: Option<ShadowPlan>) {
        let Some(plan) = plan else { return };
        self.keep(gpu, items);
        let d = gpu.device.clone();
        let cmd = gpu.cmd;
        let extent = vk::Extent2D { width: SHADOW_SIZE, height: SHADOW_SIZE };
        let draw = |fb: vk::Framebuffer, matrix: DMat4| unsafe {
            d.cmd_begin_render_pass(
                cmd,
                &vk::RenderPassBeginInfo::default()
                    .render_pass(self.pass)
                    .framebuffer(fb)
                    .render_area(vk::Rect2D { offset: vk::Offset2D::default(), extent })
                    .clear_values(&[vk::ClearValue { depth_stencil: vk::ClearDepthStencilValue { depth: 1.0, stencil: 0 } }]),
                vk::SubpassContents::INLINE,
            );
            d.cmd_set_viewport(cmd, 0, &[vk::Viewport { x: 0.0, y: 0.0, width: SHADOW_SIZE as f32, height: SHADOW_SIZE as f32, min_depth: 0.0, max_depth: 1.0 }]);
            d.cmd_set_scissor(cmd, 0, &[vk::Rect2D { offset: vk::Offset2D::default(), extent }]);
            d.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::GRAPHICS, self.pipeline);
            for it in items {
                let Some(cast) = self.meshes.get(&it.shape) else { continue };
                if cast.count == 0 {
                    continue;
                }
                let mvp = (matrix * it.world).as_mat4().to_cols_array();
                d.cmd_push_constants(cmd, self.layout, vk::ShaderStageFlags::VERTEX, 0, bytemuck::bytes_of(&mvp));
                d.cmd_bind_vertex_buffers(cmd, 0, &[cast.vb.buffer], &[0]);
                d.cmd_bind_index_buffer(cmd, cast.ib.buffer, 0, vk::IndexType::UINT32);
                d.cmd_draw_indexed(cmd, cast.count, 1, 0, 0, 0);
            }
            d.cmd_end_render_pass(cmd);
        };
        let (image, layers) = match plan.camera {
            ShadowCamera::Map2D(matrix) => {
                draw(self.framebuffer, matrix);
                (self.image, 1)
            }
            ShadowCamera::Cube { faces, .. } => {
                for i in 0..6 {
                    draw(self.cube_framebuffers[i], faces[i]);
                }
                (self.cube_image, 6)
            }
        };
        unsafe {
            let barrier = vk::ImageMemoryBarrier::default()
                .old_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                .new_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                .src_access_mask(vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE)
                .dst_access_mask(vk::AccessFlags::SHADER_READ)
                .image(image)
                .subresource_range(vk::ImageSubresourceRange::default().aspect_mask(vk::ImageAspectFlags::DEPTH).level_count(1).layer_count(layers));
            d.cmd_pipeline_barrier(cmd, vk::PipelineStageFlags::LATE_FRAGMENT_TESTS, vk::PipelineStageFlags::FRAGMENT_SHADER, vk::DependencyFlags::empty(), &[], &[], &[barrier]);
        }
    }

    /// Keeps a cast mesh per shape, rebuilding one whose mesh changed and dropping those no longer drawn.
    fn keep(&mut self, gpu: &mut Gpu, items: &[DrawItem]) {
        let live: std::collections::HashSet<NodeId> = items.iter().map(|i| i.shape).collect();
        let stale: Vec<NodeId> = self.meshes.keys().copied().filter(|k| !live.contains(k)).collect();
        for k in stale {
            if let Some(m) = self.meshes.remove(&k) {
                gpu.drop_buffer(m.vb);
                gpu.drop_buffer(m.ib);
            }
        }
        for it in items {
            let same = self.meshes.get(&it.shape).is_some_and(|m| Arc::ptr_eq(&m.source, &it.mesh));
            if same {
                continue;
            }
            if let Some(m) = self.meshes.remove(&it.shape) {
                gpu.drop_buffer(m.vb);
                gpu.drop_buffer(m.ib);
            }
            let rm = build_render_mesh(&it.mesh, &RenderMeshOptions { max_uv_sets: 0, colors: false, wireframe: false, sections: false, ..Default::default() });
            if rm.indices.is_empty() {
                continue;
            }
            let verts: Vec<[f32; 3]> = rm.positions.iter().map(|p| p.to_array()).collect();
            let vb = gpu.buffer_with(vk::BufferUsageFlags::VERTEX_BUFFER, &verts);
            let ib = gpu.buffer_with(vk::BufferUsageFlags::INDEX_BUFFER, &rm.indices);
            self.meshes.insert(it.shape, CastMesh { source: it.mesh.clone(), vb, ib, count: rm.indices.len() as u32 });
        }
    }

    pub fn release(self, gpu: &Gpu) {
        unsafe {
            let d = &gpu.device;
            d.destroy_pipeline(self.pipeline, None);
            d.destroy_pipeline_layout(self.layout, None);
            d.destroy_sampler(self.sampler, None);
            d.destroy_framebuffer(self.framebuffer, None);
            d.destroy_render_pass(self.pass, None);
            d.destroy_image_view(self.view, None);
            d.destroy_image(self.image, None);
            if let Some(a) = self.alloc {
                let _ = gpu.shared.allocator.lock().unwrap().free(a);
            }
            for f in self.cube_faces {
                d.destroy_image_view(f, None);
            }
            for f in self.cube_framebuffers {
                d.destroy_framebuffer(f, None);
            }
            d.destroy_image_view(self.cube_view, None);
            d.destroy_image(self.cube_image, None);
            if let Some(a) = self.cube_alloc {
                let _ = gpu.shared.allocator.lock().unwrap().free(a);
            }
            d.destroy_descriptor_pool(self.pool, None);
        }
        for (_, mut m) in self.meshes {
            unsafe {
                gpu.device.destroy_buffer(m.vb.buffer, None);
                gpu.device.destroy_buffer(m.ib.buffer, None);
                if let Some(a) = m.vb.alloc.take() {
                    let _ = gpu.shared.allocator.lock().unwrap().free(a);
                }
                if let Some(a) = m.ib.alloc.take() {
                    let _ = gpu.shared.allocator.lock().unwrap().free(a);
                }
            }
        }
    }
}


