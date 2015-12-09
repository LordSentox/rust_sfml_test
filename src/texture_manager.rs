use std::collections::HashMap;
use sfml::graphics::Texture;

pub struct TextureManager {
	textures: HashMap<String, Texture>
}

impl TextureManager {
	pub fn new() -> TextureManager {
		TextureManager {
			textures: HashMap::new()
		}
	}

	/// # Preload a texture
	///
	/// If the texture is found, it is saved into the HashMap and true is returned. Otherwise the
	/// function will fail and return false.
	pub fn preload(&mut self, name: &str) -> bool {
		let mut texture = match Texture::new_from_file(name) {
			Some(texture) => texture,
			None => return false
		};

		// As a default setting, the smoothness is turned off, since I found anti-aliasing to be
		// quite problematic in many situations.
		texture.set_smooth(false);

		// To avoid errors in development, this is double checked, to prevent textures from being
		// loaded twice. The check should be removed in the release version.
		assert!(!self.textures.contains_key(name));

		self.textures.insert(name.to_string(), texture);
		return true;
	}

	/// # Find a resource
	///
	/// This function looks up the specified resource, and tries to load it, if it hasn't been
	/// already. If it cannot be found, it returns None.
	pub fn find(&mut self, name: &str) -> Option<&Texture> {
		if self.textures.contains_key(name) {
			self.get(name)
		}
		else {
			self.preload(name);
			self.get(name)
		}
	}

	/// # Get a resource
	///
	/// This will return the reference to a resource, if it has already been loaded. Otherwise it
	/// will return None. It can however be called on an immutable TextureManager and is a faster.
	pub fn get(&self, name: &str) -> Option<&Texture> {
		self.textures.get(name)
	}
}
