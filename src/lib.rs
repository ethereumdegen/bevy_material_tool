use bevy::prelude::*;
use bevy::utils::HashMap;
use material_overrides::BuiltMaterialsResource;

use crate::material_overrides::MaterialOverridesResource;

pub mod material_overrides;
pub mod materials_config;



pub struct BevyMaterialToolPlugin {
    pub material_overrides_gltf_path: String ,
    pub material_types_config_path: String,
}
 
impl Plugin for BevyMaterialToolPlugin {
    fn build(&self, app: &mut App) {


    	let gltf_path = &self.material_overrides_gltf_path;
        let types_config_path = &self.material_types_config_path;

    	app 
        .init_resource::<BuiltMaterialsResource>()
    	 .insert_resource(
    	 	MaterialOverridesResource{

    	 		doodad_materials_gltf_path: gltf_path.to_string(),
                material_types_config_path: types_config_path.to_string(),
    	 		doodad_materials_gltf: None,
    	 		extracted_materials_map: HashMap::new(),
                 

    	 	}




    	 	)


    	 .add_plugins(material_overrides::material_overrides_plugin)

    	;


	}
} 