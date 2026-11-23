//! What happens to a view's picture between the scene being drawn and the window showing it: ambient occlusion from the
//! depth the scene left. One full-screen pass reads the scene's colour and depth and writes the window's image.

use ash::vk;
use glam::Mat4;

use crate::gpu::Gpu;
use crate::render::compile;

const POST_VS: &str = r#"#version 450
layout(location = 0) out vec2 uv;
void main() {
    // One triangle over the whole view.
    vec2 p = vec2(float((gl_VertexIndex << 1) & 2), float(gl_VertexIndex & 2));
    uv = p;
    gl_Position = vec4(p * 2.0 - 1.0, 0.0, 1.0);
}
"#;

/// The parts both fragment shaders share. DEPTH_TEXTURE and DEPTH_FETCH are filled in for a multisampled depth or a
/// plain one.
const POST_HEAD: &str = r#"#version 450
layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 o;
layout(set = 0, binding = 0) uniform texture2D t_scene;
layout(set = 0, binding = 1) uniform DEPTH_TEXTURE t_depth;
layout(set = 0, binding = 2) uniform sampler s_point;
layout(set = 0, binding = 3) uniform texture2D t_open;
layout(push_constant) uniform P {
    // Clip space back to the camera's.
    mat4 inv_proj;
    // Reach in scene units, strength, on (1) or off (0), orthographic (1) or not.
    vec4 ao;
    // Width, height, and one over each.
    vec4 size;
    // The background's gradient, top and bottom colours; top.w is 1 with the gradient on, bottom.w the flat grey the
    // scene was cleared to.
    vec4 bg_top;
    vec4 bg_bottom;
} pc;
// The camera's projection and the contact shadow, in a uniform buffer: the push block stays what every device allows.
layout(set = 0, binding = 4, std140) uniform Params {
    mat4 proj;
    // The direction in view space towards the shadow-casting light; w is how far its contact shadow reaches.
    vec4 contact_dir;
    // Contact shadow strength, and whether it is on (0 off).
    vec4 contact;
    // x: 1 when the Realtime renderer asks for the finer occlusion (more taps, a wider blur).
    vec4 quality;
} pp;

float depth_at(vec2 px) {
    ivec2 at = clamp(ivec2(px), ivec2(0), ivec2(pc.size.xy) - 1);
    return DEPTH_FETCH;
}

// The camera-space point a pixel shows, from its depth. Depth runs 1 at the eye to 0 far away.
vec3 point_at(vec2 px, float d) {
    vec4 p = pc.inv_proj * vec4(px * pc.size.zw * 2.0 - 1.0, d, 1.0);
    return p.xyz / p.w;
}
"#;

/// How open each pixel is: 1 with nothing near it, less in creases and where things meet.
const AO_MAIN: &str = r#"
// Noise that differs from pixel to pixel with no pattern the eye picks out.
float noise(vec2 px) {
    return fract(52.9829189 * fract(dot(px, vec2(0.06711056, 0.00583715))));
}

void main() {
    vec2 c = floor(gl_FragCoord.xy) + 0.5;
    float d = depth_at(c);
    if (d <= 0.0) {
        o = vec4(1.0);
        return;
    }
    vec3 p = point_at(c, d);
    // The surface's normal from its neighbours, taking on each axis the side nearer in depth so an edge doesn't bend it.
    float dl = depth_at(c - vec2(1.0, 0.0)), dr = depth_at(c + vec2(1.0, 0.0));
    float du = depth_at(c - vec2(0.0, 1.0)), dd = depth_at(c + vec2(0.0, 1.0));
    vec3 ax = abs(dr - d) < abs(d - dl) ? point_at(c + vec2(1.0, 0.0), dr) - p : p - point_at(c - vec2(1.0, 0.0), dl);
    vec3 ay = abs(dd - d) < abs(d - du) ? point_at(c + vec2(0.0, 1.0), dd) - p : p - point_at(c - vec2(0.0, 1.0), du);
    vec3 n = normalize(cross(ax, ay));
    vec3 to_eye = pc.ao.w > 0.5 ? vec3(0.0, 0.0, 1.0) : normalize(-p);
    if (dot(n, to_eye) < 0.0) n = -n;

    float reach = pc.ao.x;
    // How far that is on screen at this depth, kept to a size worth sampling.
    float per_px = length(point_at(c + vec2(1.0, 0.0), d) - p);
    float reach_px = clamp(reach / max(per_px, 1e-9), 3.0, 64.0);
    // On screen the reach is held to 64 pixels: close up, what is sampled is nearer than the reach asked for.
    float r = min(reach, reach_px * per_px);
    float turn = noise(c) * 6.2831853;
    float sum = 0.0;
    // Realtime takes twice the taps: a smoother term and less of the sampling's grain.
    int taps = pp.quality.x > 0.5 ? 32 : 16;
    for (int i = 0; i < 32; ++i) {
        if (i >= taps) break;
        // A spiral out from the pixel.
        float t = (float(i) + 0.5) / float(taps);
        float a = turn + t * 6.2831853 * 7.0;
        vec2 q = floor(c + vec2(cos(a), sin(a)) * reach_px * t) + 0.5;
        float dq = depth_at(q);
        if (dq <= 0.0) continue;
        vec3 v = point_at(q, dq) - p;
        float vv = dot(v, v);
        // A flat-faced mesh's neighbouring faces sit a little proud of each other: that much is not occlusion.
        float vn = dot(v, n) - 0.06 * r;
        // What is past the reach hides nothing: something far in front (a handle, another object) casts no halo.
        float f = max(1.0 - vv / (r * r), 0.0);
        sum += f * f * f * max(vn, 0.0) * r / (vv + 0.0001 * r * r);
    }
    // Contact shadows: march from the point towards the light and see whether the surface stands in the way. This is the
    // short-range shadow a shadow map's texels are too coarse for, as EEVEE's contact shadows are.
    float contact = 1.0;
    if (pp.contact.x > 0.0 && pp.contact_dir.w > 0.0) {
        vec3 dir = normalize(pp.contact_dir.xyz);
        float far = pp.contact_dir.w;
        float step_len = far / 16.0;
        float t = step_len;
        for (int i = 0; i < 16; ++i) {
            vec4 clip = pp.proj * vec4(p + dir * t, 1.0);
            if (clip.w <= 0.0) break;
            vec2 sx = clip.xy / clip.w * 0.5 + 0.5;
            if (sx.x < 0.0 || sx.x > 1.0 || sx.y < 0.0 || sx.y > 1.0) break;
            vec2 q = floor(sx * pc.size.xy) + 0.5;
            float dq = depth_at(q);
            if (dq > 0.0) {
                float zq = point_at(q, dq).z;
                float zr = (p + dir * t).z;
                if (zq - zr > far * 0.02 && zq - zr < far * 0.75) {
                    contact = 1.0 - pp.contact.x;
                    break;
                }
            }
            t += step_len;
        }
    }
    float open = pc.ao.z > 0.5 ? clamp(1.0 - pc.ao.y * sum / float(taps), 0.0, 1.0) : 1.0;
    o = vec4(clamp(open * contact, 0.0, 1.0));
}
"#;

/// The picture: the scene's colour, darkened by how closed in each pixel is. The occlusion is smoothed over the pixels
/// round about that lie on the same surface, which takes the sampling's grain out and keeps edges sharp.
const COMPOSITE_MAIN: &str = r#"
void main() {
    vec2 c = floor(gl_FragCoord.xy) + 0.5;
    vec3 color = texelFetch(sampler2D(t_scene, s_point), ivec2(c), 0).rgb;
    float d = depth_at(c);
    // Nothing drawn here (no depth, and still the colour it was cleared to): the gradient, top of the view to
    // bottom. What was drawn without depth keeps its colour.
    if (d <= 0.0 && pc.bg_top.w > 0.5 && all(lessThan(abs(color - vec3(pc.bg_bottom.w)), vec3(0.5 / 255.0)))) {
        color = mix(pc.bg_top.rgb, pc.bg_bottom.rgb, uv.y);
        o = vec4(color, 1.0);
        return;
    }
    if ((pc.ao.z < 0.5 && pp.contact.x <= 0.0) || d <= 0.0) {
        o = vec4(color, 1.0);
        return;
    }
    float z = point_at(c, d).z;
    float per_px = length(point_at(c + vec2(1.0, 0.0), d) - point_at(c, d));
    float sum = 0.0, weight = 0.0;
    // Realtime smooths over a wider square, which takes the finer sampling's remaining grain out.
    int blur = pp.quality.x > 0.5 ? 4 : 3;
    for (int y = -blur; y <= blur; ++y) {
        for (int x = -blur; x <= blur; ++x) {
            vec2 q = c + vec2(float(x), float(y));
            ivec2 at = clamp(ivec2(q), ivec2(0), ivec2(pc.size.xy) - 1);
            float dq = depth_at(q);
            if (dq <= 0.0) continue;
            // Pixels much nearer or farther are another surface: a slope across the square is not.
            float dz = abs(point_at(q, dq).z - z) / (per_px * float(blur * 2) + 1e-9);
            float w = exp(-float(x * x + y * y) / float(blur * blur * 2)) * max(1.0 - dz, 0.0);
            sum += texelFetch(sampler2D(t_open, s_point), at, 0).r * w;
            weight += w;
        }
    }
    float open = weight > 0.0 ? sum / weight : 1.0;
    o = vec4(color * open, 1.0);
}
"#;

/// A view's post effects and how strong they are.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PostSettings {
    pub ambient_occlusion: bool,
    /// How far a surface is shadowed by what is near it, in scene units.
    pub ao_radius: f32,
    pub ao_intensity: f32,
    /// The background's gradient, top and bottom, when it is on.
    pub gradient: Option<([f32; 3], [f32; 3])>,
    /// The grey the scene was cleared to.
    pub clear_grey: f32,
    /// The contact shadow: the view-space direction towards its light, how far it reaches and how strong it is.
    pub contact: Option<([f32; 3], f32, f32)>,
    /// The Realtime renderer's finer occlusion: more taps and a wider blur.
    pub high_quality: bool,
}

impl Default for PostSettings {
    fn default() -> Self {
        PostSettings { ambient_occlusion: false, ao_radius: 2.0, ao_intensity: 1.5, gradient: None, clear_grey: 0.36, contact: None, high_quality: false }
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct PostPush {
    inv_proj: [f32; 16],
    ao: [f32; 4],
    size: [f32; 4],
    bg_top: [f32; 4],
    bg_bottom: [f32; 4],
}

/// The projection and contact shadow, in a uniform buffer so the push block stays at the 128 bytes every device gives.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct PostParams {
    proj: [f32; 16],
    contact_dir: [f32; 4],
    contact: [f32; 4],
    quality: [f32; 4],
}

pub struct Post {
    pipeline: vk::Pipeline,
    ao_pipeline: vk::Pipeline,
    layout: vk::PipelineLayout,
    set_layout: vk::DescriptorSetLayout,
    pool: vk::DescriptorPool,
    set: vk::DescriptorSet,
    ao_set: vk::DescriptorSet,
    sampler: vk::Sampler,
    params: crate::gpu::Buffer,
}

fn fragment_source(main: &str, multisampled: bool) -> String {
    let (texture, fetch) = if multisampled { ("texture2DMS", "texelFetch(sampler2DMS(t_depth, s_point), at, 0).r") } else { ("texture2D", "texelFetch(sampler2D(t_depth, s_point), at, 0).r") };
    format!("{}{main}", POST_HEAD.replace("DEPTH_TEXTURE", texture).replace("DEPTH_FETCH", fetch))
}

impl Post {
    pub fn new(gpu: &Gpu) -> Post {
        let d = &gpu.device;
        unsafe {
            let module = |code: &[u32]| d.create_shader_module(&vk::ShaderModuleCreateInfo::default().code(code), None).unwrap();
            let vs = module(&compile(POST_VS, naga::ShaderStage::Vertex));
            let multisampled = gpu.samples != vk::SampleCountFlags::TYPE_1;
            let fs = module(&compile(&fragment_source(COMPOSITE_MAIN, multisampled), naga::ShaderStage::Fragment));
            let ao_fs = module(&compile(&fragment_source(AO_MAIN, multisampled), naga::ShaderStage::Fragment));
            let bindings = [
                vk::DescriptorSetLayoutBinding::default().binding(0).descriptor_type(vk::DescriptorType::SAMPLED_IMAGE).descriptor_count(1).stage_flags(vk::ShaderStageFlags::FRAGMENT),
                vk::DescriptorSetLayoutBinding::default().binding(1).descriptor_type(vk::DescriptorType::SAMPLED_IMAGE).descriptor_count(1).stage_flags(vk::ShaderStageFlags::FRAGMENT),
                vk::DescriptorSetLayoutBinding::default().binding(2).descriptor_type(vk::DescriptorType::SAMPLER).descriptor_count(1).stage_flags(vk::ShaderStageFlags::FRAGMENT),
                vk::DescriptorSetLayoutBinding::default().binding(3).descriptor_type(vk::DescriptorType::SAMPLED_IMAGE).descriptor_count(1).stage_flags(vk::ShaderStageFlags::FRAGMENT),
                vk::DescriptorSetLayoutBinding::default().binding(4).descriptor_type(vk::DescriptorType::UNIFORM_BUFFER).descriptor_count(1).stage_flags(vk::ShaderStageFlags::FRAGMENT),
            ];
            let set_layout = d.create_descriptor_set_layout(&vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings), None).unwrap();
            let sizes = [
                vk::DescriptorPoolSize::default().ty(vk::DescriptorType::SAMPLED_IMAGE).descriptor_count(6),
                vk::DescriptorPoolSize::default().ty(vk::DescriptorType::SAMPLER).descriptor_count(2),
                vk::DescriptorPoolSize::default().ty(vk::DescriptorType::UNIFORM_BUFFER).descriptor_count(2),
            ];
            let pool = d.create_descriptor_pool(&vk::DescriptorPoolCreateInfo::default().max_sets(2).pool_sizes(&sizes), None).unwrap();
            let layouts = [set_layout];
            let set = d.allocate_descriptor_sets(&vk::DescriptorSetAllocateInfo::default().descriptor_pool(pool).set_layouts(&layouts)).unwrap()[0];
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
                .unwrap();
            let params = gpu.buffer_with(vk::BufferUsageFlags::UNIFORM_BUFFER, &[PostParams { proj: glam::Mat4::IDENTITY.to_cols_array(), contact_dir: [0.0; 4], contact: [0.0; 4], quality: [0.0; 4] }]);
            let push = [vk::PushConstantRange::default().stage_flags(vk::ShaderStageFlags::FRAGMENT).offset(0).size(std::mem::size_of::<PostPush>() as u32)];
            let layout = d.create_pipeline_layout(&vk::PipelineLayoutCreateInfo::default().set_layouts(&layouts).push_constant_ranges(&push), None).unwrap();
            let stages = [
                vk::PipelineShaderStageCreateInfo::default().stage(vk::ShaderStageFlags::VERTEX).module(vs).name(c"main"),
                vk::PipelineShaderStageCreateInfo::default().stage(vk::ShaderStageFlags::FRAGMENT).module(fs).name(c"main"),
            ];
            let vin = vk::PipelineVertexInputStateCreateInfo::default();
            let ia = vk::PipelineInputAssemblyStateCreateInfo::default().topology(vk::PrimitiveTopology::TRIANGLE_LIST);
            let vp = vk::PipelineViewportStateCreateInfo::default().viewport_count(1).scissor_count(1);
            let rs = vk::PipelineRasterizationStateCreateInfo::default().polygon_mode(vk::PolygonMode::FILL).cull_mode(vk::CullModeFlags::NONE).line_width(1.0);
            let ms = vk::PipelineMultisampleStateCreateInfo::default().rasterization_samples(vk::SampleCountFlags::TYPE_1);
            let blend = [vk::PipelineColorBlendAttachmentState::default().blend_enable(false).color_write_mask(vk::ColorComponentFlags::RGBA)];
            let cb = vk::PipelineColorBlendStateCreateInfo::default().attachments(&blend);
            let dyns = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
            let dy = vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dyns);
            let info = vk::GraphicsPipelineCreateInfo::default()
                .stages(&stages)
                .vertex_input_state(&vin)
                .input_assembly_state(&ia)
                .viewport_state(&vp)
                .rasterization_state(&rs)
                .multisample_state(&ms)
                .color_blend_state(&cb)
                .dynamic_state(&dy)
                .layout(layout)
                .render_pass(gpu.post_pass)
                .subpass(0);
            let pipeline = d.create_graphics_pipelines(vk::PipelineCache::null(), &[info], None).map_err(|e| e.1).unwrap()[0];
            // The occlusion pass: the same triangle, its own fragment shader, into the occlusion image.
            let ao_stages = [
                vk::PipelineShaderStageCreateInfo::default().stage(vk::ShaderStageFlags::VERTEX).module(vs).name(c"main"),
                vk::PipelineShaderStageCreateInfo::default().stage(vk::ShaderStageFlags::FRAGMENT).module(ao_fs).name(c"main"),
            ];
            let ao_info = info.stages(&ao_stages).render_pass(gpu.ao_pass);
            let ao_pipeline = d.create_graphics_pipelines(vk::PipelineCache::null(), &[ao_info], None).map_err(|e| e.1).unwrap()[0];
            // The occlusion pass can't read the image it writes: its set has the scene's colour in that place.
            let ao_set = d.allocate_descriptor_sets(&vk::DescriptorSetAllocateInfo::default().descriptor_pool(pool).set_layouts(&layouts)).unwrap()[0];
            d.destroy_shader_module(vs, None);
            d.destroy_shader_module(fs, None);
            d.destroy_shader_module(ao_fs, None);
            Post { pipeline, ao_pipeline, layout, set_layout, pool, set, ao_set, sampler, params }
        }
    }

    /// Points the sets at the scene's images. Call with no frame in flight (before the frame draws).
    pub fn bind_images(&self, gpu: &Gpu) {
        if gpu.scene_view == vk::ImageView::null() {
            return;
        }
        // Written each frame: a view remade at another size can come back under the handle the old one had.
        let color = [vk::DescriptorImageInfo::default().image_view(gpu.scene_view).image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)];
        let depth = [vk::DescriptorImageInfo::default().image_view(gpu.depth_view).image_layout(vk::ImageLayout::DEPTH_STENCIL_READ_ONLY_OPTIMAL)];
        let point = [vk::DescriptorImageInfo::default().sampler(self.sampler)];
        let open = [vk::DescriptorImageInfo::default().image_view(gpu.ao_view).image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)];
        let params = [vk::DescriptorBufferInfo::default().buffer(self.params.buffer).offset(0).range(std::mem::size_of::<PostParams>() as u64)];
        let mut writes = Vec::new();
        for (set, last) in [(self.set, &open), (self.ao_set, &color)] {
            writes.push(vk::WriteDescriptorSet::default().dst_set(set).dst_binding(0).descriptor_type(vk::DescriptorType::SAMPLED_IMAGE).image_info(&color));
            writes.push(vk::WriteDescriptorSet::default().dst_set(set).dst_binding(1).descriptor_type(vk::DescriptorType::SAMPLED_IMAGE).image_info(&depth));
            writes.push(vk::WriteDescriptorSet::default().dst_set(set).dst_binding(2).descriptor_type(vk::DescriptorType::SAMPLER).image_info(&point));
            writes.push(vk::WriteDescriptorSet::default().dst_set(set).dst_binding(3).descriptor_type(vk::DescriptorType::SAMPLED_IMAGE).image_info(last));
            writes.push(vk::WriteDescriptorSet::default().dst_set(set).dst_binding(4).descriptor_type(vk::DescriptorType::UNIFORM_BUFFER).buffer_info(&params));
        }
        unsafe { gpu.device.update_descriptor_sets(&writes, &[]) };
    }

    fn push(&self, gpu: &Gpu, proj: Mat4, orthographic: bool, s: &PostSettings) -> PostPush {
        self.write_params(proj, s);
        let (w, h) = (gpu.extent.width.max(1) as f32, gpu.extent.height.max(1) as f32);
        PostPush {
            inv_proj: proj.inverse().to_cols_array(),
            ao: [s.ao_radius.max(1e-4), s.ao_intensity.max(0.0), if s.ambient_occlusion { 1.0 } else { 0.0 }, if orthographic { 1.0 } else { 0.0 }],
            size: [w, h, 1.0 / w, 1.0 / h],
            bg_top: match s.gradient { Some((t, _)) => [t[0], t[1], t[2], 1.0], None => [0.0; 4] },
            bg_bottom: match s.gradient { Some((_, b)) => [b[0], b[1], b[2], s.clear_grey], None => [0.0, 0.0, 0.0, s.clear_grey] },
        }
    }

    /// The projection and contact shadow, into the uniform buffer the shaders read.
    fn write_params(&self, proj: Mat4, s: &PostSettings) {
        let data = PostParams {
            proj: proj.to_cols_array(),
            contact_dir: match s.contact { Some((d, reach, _)) => [d[0], d[1], d[2], reach.max(0.0)], None => [0.0; 4] },
            contact: match s.contact { Some((_, _, strength)) => [strength.max(0.0), 0.0, 0.0, 0.0], None => [0.0; 4] },
            quality: [if s.high_quality { 1.0 } else { 0.0 }, 0.0, 0.0, 0.0],
        };
        let bytes: &[u8] = bytemuck::bytes_of(&data);
        if let Some(p) = self.params.alloc.as_ref().and_then(|a| a.mapped_ptr()) {
            unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), p.as_ptr() as *mut u8, bytes.len()) };
        }
    }

    fn pass(&self, gpu: &Gpu, pipeline: vk::Pipeline, set: vk::DescriptorSet, push: &PostPush) {
        unsafe {
            let d = &gpu.device;
            d.cmd_bind_pipeline(gpu.cmd, vk::PipelineBindPoint::GRAPHICS, pipeline);
            d.cmd_bind_descriptor_sets(gpu.cmd, vk::PipelineBindPoint::GRAPHICS, self.layout, 0, &[set], &[]);
            d.cmd_push_constants(gpu.cmd, self.layout, vk::ShaderStageFlags::FRAGMENT, 0, bytemuck::bytes_of(push));
            d.cmd_draw(gpu.cmd, 3, 1, 0, 0);
        }
    }

    /// Works out the occlusion. Between `Gpu::begin_occlusion` and `Gpu::begin_post`. Nothing is drawn with it off.
    pub fn draw_occlusion(&self, gpu: &Gpu, proj: Mat4, orthographic: bool, s: &PostSettings) {
        if s.ambient_occlusion || s.contact.is_some() {
            self.pass(gpu, self.ao_pipeline, self.ao_set, &self.push(gpu, proj, orthographic, s));
        }
    }

    /// Draws the scene's picture onto the window's image. Between `Gpu::begin_post` and `Gpu::end_frame`.
    pub fn draw(&self, gpu: &Gpu, proj: Mat4, orthographic: bool, s: &PostSettings) {
        self.pass(gpu, self.pipeline, self.set, &self.push(gpu, proj, orthographic, s));
    }

    pub fn destroy(&mut self, gpu: &Gpu) {
        unsafe {
            let d = &gpu.device;
            d.destroy_pipeline(self.pipeline, None);
            d.destroy_pipeline(self.ao_pipeline, None);
            d.destroy_pipeline_layout(self.layout, None);
            d.destroy_descriptor_pool(self.pool, None);
            d.destroy_descriptor_set_layout(self.set_layout, None);
            d.destroy_sampler(self.sampler, None);
            d.destroy_buffer(self.params.buffer, None);
            if let Some(a) = self.params.alloc.take() {
                let _ = gpu.shared.allocator.lock().unwrap().free(a);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_post_shaders_compile_for_one_sample_and_for_several() {
        assert!(!compile(POST_VS, naga::ShaderStage::Vertex).is_empty());
        for multisampled in [false, true] {
            for main in [AO_MAIN, COMPOSITE_MAIN] {
                assert!(!compile(&fragment_source(main, multisampled), naga::ShaderStage::Fragment).is_empty(), "multisampled {multisampled}");
            }
        }
    }

    #[test]
    fn the_push_block_is_what_the_shader_declares() {
        // In reverse projection, the occlusion, the size and the two background colours: the 128 bytes every device gives.
        // The projection and contact shadow ride in a uniform buffer, so this stays at that minimum.
        assert_eq!(std::mem::size_of::<PostPush>(), 128);
    }
}
