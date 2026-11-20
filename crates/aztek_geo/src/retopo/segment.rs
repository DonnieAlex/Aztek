//! Splitting the surface into regions whose borders the new faces follow: by shape, Variational Shape Approximation
//! (Cohen-Steiner, Alliez and Desbrun, 2004) with normal proxies, and by colour, k-means in Lab then connected parts.

use super::tri::Tri;
use glam::DVec3;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq)]
struct Item {
    cost: f64,
    face: u32,
    label: u32,
}
impl Eq for Item {}
impl PartialOrd for Item {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}
impl Ord for Item {
    fn cmp(&self, o: &Self) -> Ordering {
        o.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal).then(o.face.cmp(&self.face))
    }
}

/// Regions of nearly one normal each, about `k` of them. Returns a label per triangle.
pub fn by_shape(t: &Tri, k: usize, iterations: usize) -> Vec<u32> {
    let nf = t.f.len();
    if nf == 0 {
        return Vec::new();
    }
    let adj = t.face_adjacency();
    let k = k.clamp(1, nf);
    // Seeds spread over the surface: each the triangle whose normal is furthest from every seed so far, weighted by
    // distance, so a box's six sides each get one before any gets two.
    let mut seeds: Vec<u32> = vec![largest(t)];
    let centroid = |f: usize| (t.v[t.f[f][0] as usize] + t.v[t.f[f][1] as usize] + t.v[t.f[f][2] as usize]) / 3.0;
    let diag = bounds_diag(t).max(1e-12);
    let mut mind = vec![f64::MAX; nf];
    while seeds.len() < k {
        let s = *seeds.last().unwrap() as usize;
        let mut best = (f64::MIN, 0u32);
        for f in 0..nf {
            let dn = (t.fnormal[f] - t.fnormal[s]).length_squared();
            let dp = (centroid(f) - centroid(s)).length() / diag;
            mind[f] = mind[f].min(dn + 0.25 * dp);
            let score = mind[f] * t.farea[f].sqrt();
            if score > best.0 {
                best = (score, f as u32);
            }
        }
        if best.0 <= 1e-12 {
            break;
        }
        seeds.push(best.1);
    }
    let mut proxies: Vec<DVec3> = seeds.iter().map(|&s| t.fnormal[s as usize]).collect();
    let mut label = vec![u32::MAX; nf];
    for _ in 0..iterations.max(1) {
        label.iter_mut().for_each(|l| *l = u32::MAX);
        let mut heap = BinaryHeap::new();
        for (r, &s) in seeds.iter().enumerate() {
            label[s as usize] = r as u32;
            for &g in &adj[s as usize] {
                heap.push(Item { cost: err(t, g as usize, proxies[r]), face: g, label: r as u32 });
            }
        }
        while let Some(it) = heap.pop() {
            if label[it.face as usize] != u32::MAX {
                continue;
            }
            label[it.face as usize] = it.label;
            for &g in &adj[it.face as usize] {
                if label[g as usize] == u32::MAX {
                    heap.push(Item { cost: err(t, g as usize, proxies[it.label as usize]), face: g, label: it.label });
                }
            }
        }
        // Parts no seed reached (separate shells) take a label of their own.
        for f in 0..nf {
            if label[f] == u32::MAX {
                let r = proxies.len() as u32;
                proxies.push(t.fnormal[f]);
                seeds.push(f as u32);
                flood(&adj, &mut label, f, r);
            }
        }
        // New proxies, then each region's seed is its triangle closest to the proxy.
        let mut sum = vec![DVec3::ZERO; proxies.len()];
        for f in 0..nf {
            sum[label[f] as usize] += t.fnormal[f] * t.farea[f];
        }
        for (r, s) in sum.iter().enumerate() {
            proxies[r] = s.try_normalize().unwrap_or(proxies[r]);
        }
        let mut best = vec![(f64::MAX, 0u32); proxies.len()];
        for f in 0..nf {
            let r = label[f] as usize;
            let e = (t.fnormal[f] - proxies[r]).length_squared();
            if e < best[r].0 {
                best[r] = (e, f as u32);
            }
        }
        seeds = best.iter().map(|b| b.1).collect();
    }
    absorb_small(t, &adj, &mut label, 0.002);
    label
}

fn err(t: &Tri, f: usize, proxy: DVec3) -> f64 {
    (t.fnormal[f] - proxy).length_squared() * t.farea[f]
}

fn largest(t: &Tri) -> u32 {
    t.farea.iter().enumerate().max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(Ordering::Equal)).map(|x| x.0 as u32).unwrap_or(0)
}

fn bounds_diag(t: &Tri) -> f64 {
    let mut lo = DVec3::splat(f64::MAX);
    let mut hi = DVec3::splat(f64::MIN);
    for p in &t.v {
        lo = lo.min(*p);
        hi = hi.max(*p);
    }
    (hi - lo).length()
}

fn flood(adj: &[Vec<u32>], label: &mut [u32], start: usize, r: u32) {
    let mut stack = vec![start];
    label[start] = r;
    while let Some(f) = stack.pop() {
        for &g in &adj[f] {
            if label[g as usize] == u32::MAX {
                label[g as usize] = r;
                stack.push(g as usize);
            }
        }
    }
}

/// Connected parts of each label smaller than `fraction` of the area join the neighbour they share most border with.
fn absorb_small(t: &Tri, adj: &[Vec<u32>], label: &mut [u32], fraction: f64) {
    let total: f64 = t.farea.iter().sum();
    for _ in 0..4 {
        // Connected components.
        let nf = label.len();
        let mut comp = vec![u32::MAX; nf];
        let mut comps: Vec<(f64, Vec<u32>)> = Vec::new();
        for f in 0..nf {
            if comp[f] != u32::MAX {
                continue;
            }
            let c = comps.len() as u32;
            let mut stack = vec![f];
            comp[f] = c;
            let mut members = Vec::new();
            let mut area = 0.0;
            while let Some(x) = stack.pop() {
                members.push(x as u32);
                area += t.farea[x];
                for &g in &adj[x] {
                    if comp[g as usize] == u32::MAX && label[g as usize] == label[x] {
                        comp[g as usize] = c;
                        stack.push(g as usize);
                    }
                }
            }
            comps.push((area, members));
        }
        let mut changed = false;
        for (area, members) in &comps {
            if *area >= total * fraction {
                continue;
            }
            let own = label[members[0] as usize];
            let mut votes: std::collections::HashMap<u32, usize> = Default::default();
            for &f in members {
                for &g in &adj[f as usize] {
                    let l = label[g as usize];
                    if l != own {
                        *votes.entry(l).or_default() += 1;
                    }
                }
            }
            if let Some((&l, _)) = votes.iter().max_by_key(|x| (*x.1, std::cmp::Reverse(*x.0))) {
                for &f in members {
                    label[f as usize] = l;
                }
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
}

/// Regions of one colour: the triangles' colours (their own, else their points') clustered in Lab, then split into
/// connected parts, small parts absorbed. Returns a label per triangle, or None when there's no colour or only one.
pub fn by_color(t: &Tri, max_k: usize) -> Option<Vec<u32>> {
    let nf = t.f.len();
    let col: Vec<[f32; 3]> = if !t.face_color.is_empty() {
        t.face_color.clone()
    } else if !t.color.is_empty() {
        t.f.iter()
            .map(|f| {
                let c = f.map(|p| t.color[p as usize]);
                [0, 1, 2].map(|i| (c[0][i] + c[1][i] + c[2][i]) / 3.0)
            })
            .collect()
    } else {
        return None;
    };
    let lab: Vec<DVec3> = col.iter().map(|c| to_lab(*c)).collect();
    // How many colours: k-means for k = 1..max_k, keeping the last k that cut the error by a fifth or more.
    let weights = &t.farea;
    let mut best_labels = vec![0u32; nf];
    let mut prev_err = f64::MAX;
    for k in 1..=max_k.max(1) {
        let (labels, e) = kmeans(&lab, weights, k);
        if k > 1 && e > prev_err * 0.8 {
            break;
        }
        prev_err = e;
        best_labels = labels;
        if e < 1e-6 {
            break;
        }
    }
    if best_labels.iter().all(|&l| l == best_labels[0]) {
        return None;
    }
    let adj = t.face_adjacency();
    absorb_small(t, &adj, &mut best_labels, 0.003);
    Some(best_labels)
}

fn kmeans(x: &[DVec3], w: &[f64], k: usize) -> (Vec<u32>, f64) {
    let n = x.len();
    // k-means++ seeding, deterministic: the furthest point each time.
    let mut centers = vec![x[0]];
    while centers.len() < k {
        let mut best = (f64::MIN, 0usize);
        for i in 0..n {
            let d = centers.iter().map(|c| (x[i] - *c).length_squared()).fold(f64::MAX, f64::min) * w[i].max(1e-12);
            if d > best.0 {
                best = (d, i);
            }
        }
        centers.push(x[best.1]);
    }
    let mut labels = vec![0u32; n];
    for _ in 0..20 {
        let mut moved = false;
        for i in 0..n {
            let mut b = (f64::MAX, 0u32);
            for (c, ctr) in centers.iter().enumerate() {
                let d = (x[i] - *ctr).length_squared();
                if d < b.0 {
                    b = (d, c as u32);
                }
            }
            if labels[i] != b.1 {
                labels[i] = b.1;
                moved = true;
            }
        }
        let mut sum = vec![(DVec3::ZERO, 0.0); k];
        for i in 0..n {
            sum[labels[i] as usize].0 += x[i] * w[i];
            sum[labels[i] as usize].1 += w[i];
        }
        for c in 0..k {
            if sum[c].1 > 0.0 {
                centers[c] = sum[c].0 / sum[c].1;
            }
        }
        if !moved {
            break;
        }
    }
    let e: f64 = (0..n).map(|i| (x[i] - centers[labels[i] as usize]).length_squared() * w[i]).sum();
    (labels, e)
}

fn to_lab(c: [f32; 3]) -> DVec3 {
    let (r, g, b) = (c[0].max(0.0) as f64, c[1].max(0.0) as f64, c[2].max(0.0) as f64);
    let x = (0.4124 * r + 0.3576 * g + 0.1805 * b) / 0.95047;
    let y = 0.2126 * r + 0.7152 * g + 0.0722 * b;
    let z = (0.0193 * r + 0.1192 * g + 0.9505 * b) / 1.08883;
    let f = |t: f64| if t > 0.008856 { t.cbrt() } else { 7.787 * t + 16.0 / 116.0 };
    let (fx, fy, fz) = (f(x), f(y), f(z));
    DVec3::new(116.0 * fy - 16.0, 500.0 * (fx - fy), 200.0 * (fy - fz))
}
