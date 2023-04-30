#include "dither.wgsl"
#include "tonemap.wgsl"
#include "atmosphere.wgsl"

fn fresnelSchlick(cosTheta: f32, F0: vec3<f32>) -> vec3<f32> {
    return F0 + (1.0 - F0) * pow(clamp(1.0 - cosTheta, 0.0, 1.0), 5.0);
}

fn fresnelSchlickRoughness(cosTheta: f32, F0: vec3<f32>, roughness: f32) -> vec3<f32> {
    return F0 + (max(vec3(1.0 - roughness), F0) - F0) * pow(clamp(1.0 - cosTheta, 0.0, 1.0), 5.0);
}

fn DistributionGGX(NdotH: f32, roughness: f32) -> f32 {
    let a: f32      = roughness*roughness;
    let a2: f32     = a*a;
    let NdotH2: f32 = NdotH*NdotH;

    let num: f32   = a2;
    var denom: f32 = (NdotH2 * (a2 - 1.0) + 1.0);
    denom = PI * denom * denom;

    return num / denom;
}

fn GeometrySchlickGGX(NdotV: f32, roughness: f32) -> f32 {
    let r: f32 = (roughness + 1.0);
    let k: f32 = (r*r) / 8.0;

    let num: f32   = NdotV;
    let denom: f32 = NdotV * (1.0 - k) + k;

    return num / denom;
}

fn GeometrySmith(NdotV: f32, NdotL: f32, roughness: f32) -> f32 {
    let ggx2: f32  = GeometrySchlickGGX(NdotV, roughness);
    let ggx1: f32  = GeometrySchlickGGX(NdotL, roughness);

    return ggx1 * ggx2;
}

fn render(sun: vec3<f32>,
          V: vec3<f32>,
          position: vec2<f32>,
          normal: vec3<f32>,
          albedo: vec3<f32>,
          F0: vec3<f32>,
          F_spec: vec3<f32>,
          sun_col: vec3<f32>,
          irradiance_diffuse: vec3<f32>,
          specular: vec3<f32>,
          metallic: f32,
          roughness: f32,
          shadow_v: f32,
          ssao: f32,
          depth: f32
          ) -> vec3<f32>  {
    let H: vec3<f32> = normalize(sun + V);
    let NdotL: f32 = max(dot(normal, sun), 0.0);
    let NdotV: f32 = max(dot(normal, V), 0.0);

    let NDF: f32 = DistributionGGX(dot(normal, H), roughness);
    let G: f32   = GeometrySmith(NdotV, NdotL, roughness);
    let F: vec3<f32>  = fresnelSchlick(max(dot(H, V), 0.0), F0);

    let kS: vec3<f32> = F;
    var kD: vec3<f32> = vec3(1.0) - kS;

    kD *= 1.0 - vec3(metallic);


    let numerator: vec3<f32>      = NDF * G * F;
    let denominator: f32    = 4.0 * max(NdotV, 0.0) * NdotL  + 0.0001;
    let specular_light: vec3<f32> = numerator / denominator;

    let Lo: vec3<f32> = (kD * albedo * ssao / PI + specular_light) * (4.0 * shadow_v * sun_col) * NdotL;

    let dkS: vec3<f32> = F_spec;
    var dkD: vec3<f32> = 1.0 - dkS;
    dkD *= 1.0 - vec3(metallic);

    let ambient: vec3<f32> = (0.2 * dkD * irradiance_diffuse * albedo + specular) * ssao;
    let atmosphere: vec3<f32> = atmosphere(-V, sun, depth * 0.2);

    var color: vec3<f32>   = ambient + Lo + atmosphere;

    color = tonemap(color);

    color += dither(position);

    return color;
}