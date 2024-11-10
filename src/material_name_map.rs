

use bevy::{prelude::*, reflect};
use bevy::utils::HashMap; 



/*


replace this with 


fn find_top_material_and_mesh(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>,
    mat_query: Query<(
        &MeshMaterial3d<StandardMaterial>,
        &Mesh3d,
        &GltfMaterialName,
    )>,
) {
    for (mat_handle, mesh_handle, name) in mat_query.iter() {
        // locate the material and associated submesh by name
        if name.0 == "Top" {
            if let Some(material) = materials.get_mut(mat_handle) {
                // ...
            }

            if let Some(mesh) = meshes.get_mut(mesh_handle) {
                // ...
            }
        }
    }
}

*/

// DEPRECATED
/* 
pub fn material_name_map_plugin(app: &mut App) {
    app 	


    .init_resource::<MaterialNameMapResource>() 
    .register_type::< MaterialNameMapResource >()
     .register_type::< MaterialMetadataName >()
    .add_systems(Update, 

    	 ( 
    	 	//build_material_name_map ,
    	 //	add_material_metadata_name_components,   //use bevy 0.15 new feature for this ..


    		).chain()


    	)


    ;
} */

#[derive(Reflect,Resource,Default)] 
#[reflect(Resource)]
pub struct MaterialNameMapResource {

	pub material_name_map: HashMap< AssetId<StandardMaterial>, String  >

}


#[derive(Reflect,Component)]
#[reflect(Component)]
pub struct MaterialMetadataName(pub String);    


fn add_material_metadata_name_components(

	mut commands: Commands, 

	entity_query:  Query< (Entity, &MeshMaterial3d::<StandardMaterial>), Added<MeshMaterial3d::<StandardMaterial>> >,

	 material_name_map_resource: Res <MaterialNameMapResource>,

) {

	for (entity,mat_handle) in entity_query.iter(){

		if let Some(mat_name)  = material_name_map_resource.material_name_map.get  ( &mat_handle.id() ) {


			   commands.entity(entity).try_insert(
				MaterialMetadataName(mat_name.clone()) 
				);
		}



	}



}

fn build_material_name_map(
	  mut asset_ready_event: EventReader<AssetEvent<Gltf>>,
     mut material_name_map_resource: ResMut<MaterialNameMapResource>,
     //mut next_state: ResMut<NextState<MaterialOverridesLoadingState>>,

     gltf_assets: Res<Assets<Gltf>>,
 
	 
){

	for evt in asset_ready_event.read(){
		match evt {
    
			    AssetEvent::LoadedWithDependencies { id } => {
			    	 
			    		let Some(gltf_asset) = gltf_assets.get( *id ) else {continue};

			    		for (material_name, material_handle) in &gltf_asset.named_materials {
			    			
			    			let material_asset_id = material_handle.id();
			    			material_name_map_resource.material_name_map.insert( material_asset_id, material_name.clone().to_string()   );
			    			 	
			    			// info!("registered material name {:?}", material_name );
			    		} 
			    		 
			    }

			    _ => {}
			}



	} 

}

