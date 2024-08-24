use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::material_overrides::MaterialOverridesResource;

pub mod material_overrides;



pub struct MaterialOverridesPlugin {
    pub material_overrides_gltf_path: String 
}
 
impl Plugin for MaterialOverridesPlugin {
    fn build(&self, app: &mut App) {


    	let path = &self.material_overrides_gltf_path;

    	app 
    	 .insert_resource(
    	 	MaterialOverridesResource{
    	 		
    	 		doodad_materials_gltf_path: path.to_string(),
    	 		doodad_materials_gltf: None,
    	 		extracted_materials_map: HashMap::new(),

    	 	}



    	 	)

    	;


	}
} 