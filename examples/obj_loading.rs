use error_stack::{Report, Result, ResultExt};
use text_3d_graphics::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("an error occured in main fn")]
struct MainError;

fn main() -> Result<(), MainError> {
    let (models, materials) = tobj::load_obj(
        "./resources/Pikachu.obj",
        &tobj::LoadOptions {
            triangulate: true,
            ..Default::default()
        },
    )
    .change_context_lazy(|| MainError)
    .attach_printable_lazy(|| "Failed to load obj")?;
    let materials = materials
        .change_context_lazy(|| MainError)
        .attach_printable_lazy(|| "Obj did not come with materials")?;

    for model in models.iter() {
        println!(
            "\nTriangle count: {}    MaterialId: {:?}",
            model.mesh.indices.len() / 3,
            model.mesh.material_id
        );

        if let Some(mat_i) = model.mesh.material_id {
            let material = &materials[mat_i];
            println!(
                "Material Name: {}\nDiffuse: {:?}  Ambient: {:?}   Specular: {:?}",
                material.name, material.diffuse, material.ambient, material.specular
            );
        }
        let simple: SimpleMesh = (model.mesh.clone(), materials.as_slice()).into();
        println!("Simple Triangle Count: {}", simple.triangles.len());
        println!("Simple Triangle Color: {:?}", simple.triangles[0].color);
    }

    Ok(())
}
