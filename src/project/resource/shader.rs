use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::project::{
    ResourceId, ResourceTrait, formatter::format_gamemaker_json, resource::ResourceBase,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GMShader {
    #[serde(rename = "$GMShader")]
    resource_tag: String,
    #[serde(flatten)]
    pub base: ResourceBase,
    parent: ResourceId,
    #[serde(rename = "isCompatibility")]
    is_compatibility: bool,
    #[serde(rename = "isDnd")]
    is_dnd: bool,
}

impl Default for GMShader {
    fn default() -> Self {
        Self {
            resource_tag: "v1".into(),
            base: ResourceBase::new("Shader1", "GMShader"),
            is_compatibility: false,
            is_dnd: false,
            parent: ResourceId::default(),
        }
    }
}

impl ResourceTrait for GMShader {
    fn name(&self) -> &str {
        &self.base.name
    }

    fn save(&self, path: &std::path::Path) -> std::io::Result<()> {
        let value = serde_json::to_value(self).expect("Failed to serialize Shader");
        let json = format_gamemaker_json(&value);
        fs::write(path, json)?;

        self.ensure_shader_files_exists(path)?;

        Ok(())
    }

    fn default_path(&self) -> String {
        format!("shaders/{}/{}.yy", self.base.name, self.base.name)
    }
}

impl GMShader {
    pub fn new(name: &str, parent: ResourceId) -> Self {
        Self {
            base: ResourceBase::new(name, "GMShader"),
            parent,
            ..Default::default()
        }
    }

    pub fn load(value: Value) -> std::io::Result<Self> {
        let shader = serde_json::from_value(value).expect("Failed to deserialize Shader");
        Ok(shader)
    }

    pub fn get_vsh_path_from_script_path(
        &self,
        script_path: &std::path::Path,
    ) -> std::path::PathBuf {
        script_path.with_file_name(format!("{}.vsh", self.base.name))
    }

    pub fn get_fsh_path_from_script_path(
        &self,
        script_path: &std::path::Path,
    ) -> std::path::PathBuf {
        script_path.with_file_name(format!("{}.fsh", self.base.name))
    }

    pub fn ensure_shader_files_exists(&self, path: &std::path::Path) -> std::io::Result<()> {
        let vsh_path = self.get_vsh_path_from_script_path(path);
        let fsh_path = self.get_fsh_path_from_script_path(path);
        if !vsh_path.exists() {
            let mut file = fs::File::create(&vsh_path)?;
            use std::io::Write;
            let shader = r#"//
// Simple passthrough vertex shader
//
attribute vec3 in_Position;                  // (x,y,z)
//attribute vec3 in_Normal;                  // (x,y,z)     unused in this shader.
attribute vec4 in_Colour;                    // (r,g,b,a)
attribute vec2 in_TextureCoord;              // (u,v)

varying vec2 v_vTexcoord;
varying vec4 v_vColour;

void main()
{
    vec4 object_space_pos = vec4( in_Position.x, in_Position.y, in_Position.z, 1.0);
    gl_Position = gm_Matrices[MATRIX_WORLD_VIEW_PROJECTION] * object_space_pos;
    
    v_vColour = in_Colour;
    v_vTexcoord = in_TextureCoord;
}
"#;
            writeln!(file, "{}", shader.to_string())?;
        }
        if !fsh_path.exists() {
            let mut file = fs::File::create(&fsh_path)?;
            use std::io::Write;
            let shader = r#"//
// Simple passthrough fragment shader
//
varying vec2 v_vTexcoord;
varying vec4 v_vColour;

void main()
{
    gl_FragColor = v_vColour * texture2D( gm_BaseTexture, v_vTexcoord );
}
"#;
            writeln!(file, "{}", shader.to_string())?;
        }
        Ok(())
    }

    pub fn get_vsh_code(&self, path: &std::path::Path) -> std::io::Result<String> {
        let vsh_path = self.get_vsh_path_from_script_path(path);
        let code = fs::read_to_string(vsh_path)?;
        Ok(code)
    }

    pub fn get_fsh_code(&self, path: &std::path::Path) -> std::io::Result<String> {
        let fsh_path = self.get_fsh_path_from_script_path(path);
        let code = fs::read_to_string(fsh_path)?;
        Ok(code)
    }
}
