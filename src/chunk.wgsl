#import bevy_sprite::{
    mesh2d_vertex_output::VertexOutput
}

struct TilemapChunkMaterial {
    tile_size: u32,
};

@group(2) @binding(0) var<uniform> material: TilemapChunkMaterial;
@group(2) @binding(1) var atlas: texture_2d<f32>;
@group(2) @binding(2) var atlas_sampler: sampler;
@group(2) @binding(3) var indices: texture_2d<u32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let chunk_dimensions = vec2<f32>(textureDimensions(indices));
    let tile_uv = mesh.uv * chunk_dimensions;
    let tile_pos = clamp(vec2<u32>(floor(tile_uv)), vec2<u32>(0), vec2<u32>(chunk_dimensions - 1));
    var tile_index = textureLoad(indices, tile_pos, 0).r;
    
    if tile_index == 0 {
        discard;
    }

    tile_index -= 1;    

    let local_uv = fract(tile_uv);

    let atlas_dimensions = vec2<f32>(textureDimensions(atlas));
    let tiles_per_row = u32(atlas_dimensions.x) / material.tile_size;
    let atlas_pos = vec2<f32>(
        f32(tile_index % tiles_per_row),
        f32(tile_index / tiles_per_row)
    );
    
    let tile_size = vec2<f32>(f32(material.tile_size), f32(material.tile_size));
    let normalized_atlas_pos = atlas_pos * tile_size / atlas_dimensions;
    let atlas_uv = normalized_atlas_pos + local_uv * (tile_size / atlas_dimensions);

    return textureSample(atlas, atlas_sampler, atlas_uv);
}