use na;
pub struct Particle {
    pub position: na::Vec3<f32>,
    pub spawntime: f32
}

pub struct ParticleSystem {
    pub particles: Vec<Particle>,
    pub particles_count: u32,

    pub lifetime: f32,
}
impl ParticleSystem {
    pub fn update(&mut self, curtime: f32, _dt: f32) {
        let dead = curtime - self.lifetime;

        for (idx, particle) in self.particles.iter_mut().rev().take(self.particles_count as usize).enumerate() {
            if particle.spawntime < dead {
                self.particles_count = idx as u32;
                break;
            }
        }
    }
    pub fn add(&mut self, particle: Particle) {
        self.particles_count += 1;
        let particles_capacity = self.particles.len();
        if self.particles_count as usize > particles_capacity {
            self.particles.push(particle);
        } else {
            self.particles[particles_capacity - self.particles_count as usize] = particle;
        }
    }

}
