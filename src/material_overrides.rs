
 
use bevy::prelude::*;
use bevy::utils::HashMap;

//use crate::loading::EditorLoadingState;  
use bevy::scene::SceneInstanceReady; 



/*

The materials MUST finish extraction before loading in the models 

*/
pub fn material_overrides_plugin(app: &mut App) {
    app 	

    
    	.init_state::<MaterialOverridesLoadingState>()
    	 
    	  .add_systems(OnEnter(MaterialOverridesLoadingState::Extracting), load_material_overrides)

     
       .add_systems(Update, extract_material_overrides )


       .add_systems(Update, handle_material_overrides )

   

       ;
}



#[derive(Clone,Debug,PartialEq,Eq,Hash,States,Default)]
pub enum MaterialOverridesLoadingState{
	#[default]
   Init,
   Extracting,
   Complete
}




#[derive(Resource,Default)]
pub struct MaterialOverridesResource {

	pub doodad_materials_gltf_path: String,

	pub doodad_materials_gltf: Option<Handle<Gltf>>,

	pub extracted_materials_map :HashMap< String, Handle<StandardMaterial> >

}

impl MaterialOverridesResource{

	pub fn find_material_by_name(&self, mat_name: &String ) -> Option<&Handle<StandardMaterial>> {


		self.extracted_materials_map.get(mat_name)
	}

}




//attach this to signal that the material is supposed to be replaced 
#[derive(Component,Debug)]
pub struct MaterialOverrideComponent {
 
	pub material_override: String
}


/*
#[derive(Component,Debug)]
pub struct ReadyForMaterialOverride ;
*/



fn load_material_overrides(

	asset_server: ResMut<AssetServer> ,

	mut material_overrides_resource: ResMut<MaterialOverridesResource>,

	mut next_state: ResMut<NextState<MaterialOverridesLoadingState>>,


){	

	let material_overrides_path = &material_overrides_resource.doodad_materials_gltf_path;
	//let material_overrides_path = "material_overrides/doodad_material_overrides.glb";

	let doodad_materials_gltf = asset_server.load::<Gltf>( material_overrides_path  );

	material_overrides_resource.doodad_materials_gltf = Some(doodad_materials_gltf);

//	next_state.set(MaterialOverridesLoadingState::Extracting);


}


fn extract_material_overrides(
	  mut asset_ready_event: EventReader<AssetEvent<Gltf>>,
     mut material_overrides_resource: ResMut<MaterialOverridesResource>,
     mut next_state: ResMut<NextState<MaterialOverridesLoadingState>>,

     gltf_assets: Res<Assets<Gltf>>,

	 
){

	for evt in asset_ready_event.read(){
		match evt {
   
			    AssetEvent::LoadedWithDependencies { id } => {
			    	if material_overrides_resource.doodad_materials_gltf.as_ref().is_some_and(|h| h.id() == *id ){

			    		let Some(doodad_materials_gltf) = gltf_assets.get( *id ) else {continue};

			    		for (material_name, material_handle) in &doodad_materials_gltf.named_materials {
			    			//info!("extracted override material: {}", material_name.to_string());
			    			material_overrides_resource.extracted_materials_map.insert(material_name.to_string(), material_handle.clone());
			    		}


			    		next_state.set(MaterialOverridesLoadingState::Complete);

			    	}
			    }

			    _ => {}
			}



	}


	

}

fn handle_material_overrides(
	mut commands:Commands, 
	mut  scene_instance_evt_reader: EventReader<SceneInstanceReady>,  

	material_override_request_query: Query<&MaterialOverrideComponent >,

	parent_query : Query<&Parent>, 
	// name_query: Query<&Name>,
	children_query: Query<&Children>,

	material_handle_query: Query<&Handle<StandardMaterial>>,

	  mut materials: ResMut<Assets<StandardMaterial>>,


	material_overrides_resource: Res<MaterialOverridesResource>
){




    for evt in scene_instance_evt_reader.read(){

          let parent = evt.parent; //the scene 

          let Some(doodad_entity) = parent_query.get(parent).ok().map( |p| p.get() ) else {continue};

          if let Some(mat_override_request) = material_override_request_query.get(doodad_entity).ok(){

                	/*commands
	                    .entity(doodad_entity)
	                    .remove::<MaterialOverrideRequestComponent>( ); */



             	info!("about to handle material override {:?}", mat_override_request);

           //  	let Some(children) = children_query.get(doodad_entity).ok() else {continue};

             	let material_name = &mat_override_request.material_override ;


             //	for (mat_base,mat_type) in mat_override_request.material_overrides.iter() {

             		//let mat_base_name = mat_base.get_material_layer_name();
             		let Some(new_material_handle) = material_overrides_resource
             		   .find_material_by_name(&material_name) else {
             		   	warn!("could not get override material");
             		   	continue
             		     }; 

 

             		 	 for child in DescendantIter::new(&children_query, doodad_entity) {

             		 	 	if let Some( _mat_handle) = material_handle_query.get(child).ok(){
 

             		 	 		 commands
				                    .entity(child)
				                    .insert(new_material_handle.clone()); 

				                  info!("inserted new material as override");


             		 	 	}else {
             		 	 		//insert pink material 

             		 	 		let warning_material = materials.add(Color::srgb(1.0, 0.0, 0.0)) ;

             		 	 		 commands
				                    .entity(child)
				                    .insert(warning_material.clone()); 

				                  info!("inserted new material as override");


             		 	 	}
						     
						    }


				             



             	//}



          }
           

      }

}