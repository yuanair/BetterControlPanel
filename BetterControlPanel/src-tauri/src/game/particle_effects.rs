struct ParticleEffectEngine {
    particles: Vec<Box<dyn Particle>>,
}

trait Particle {}

impl ParticleEffectEngine {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
        }
    }

    pub fn add_particle(&mut self, particle: Box<dyn Particle>) {
        self.particles.push(particle);
    }

    pub fn update(&mut self, delta_time: f32) {
        for particle in &mut self.particles {
            particle.update(delta_time);
        }
    }
}
