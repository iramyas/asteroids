use macroquad::prelude::*;
use crate::asteroid::{Asteroid, AsteroidSize};
use crate::missile::Missile;
use crate::spaceship::Spaceship;

/// Trait définissant les comportements communs pour les objets du jeu.
///
/// Ce trait sert à unifier les fonctionnalités essentielles des différents objets
/// stellaires du jeu, comme les astéroïdes, les missiles ou les vaisseaux spatiaux.
pub trait StellarObject {
    /// Obtenir la position actuelle de l'objet.
    fn get_position(&self) -> Vec2;

    /// Obtenir la vitesse actuelle de l'objet.
    fn get_velocity(&self) -> Vec2;

    /// Vérifie si l'objet est actif dans le jeu.
    fn is_active(&self) -> bool;

    /// Définit l'état actif ou inactif de l'objet.
    fn set_active(&mut self, active: bool);

    /// Met à jour la position et l'état de l'objet.
    ///
    /// # Arguments
    /// * `delta_time` - Le temps écoulé depuis la dernière mise à jour (en secondes).
    fn update(&mut self, delta_time: f32);

    /// Dessine l'objet à l'écran.
    fn draw(&self);

    /// Obtenir le rayon de collision de l'objet.
    fn get_radius(&self) -> f32;

    /// Gère la collision entre cet objet et un autre.
    ///
    /// # Arguments
    /// * `other` - Une référence mutable à un autre objet stellaire.
    fn handle_collision(&mut self, other: &mut StellarObjectEnum);
}

/// Énumération unifiée pour tous les objets du jeu.
///
/// Cette énumération regroupe les différents types d'objets stellaires,
/// permettant de les gérer de manière polymorphe.
pub enum StellarObjectEnum {
    Spaceship(Spaceship),
    Asteroid(Asteroid),
    Missile(Missile),
}

impl StellarObject for StellarObjectEnum {
    /// Récupère la position de l'objet en fonction de son type.
    fn get_position(&self) -> Vec2 {
        match self {
            StellarObjectEnum::Asteroid(a) => a.get_position(),
            StellarObjectEnum::Missile(m) => m.position,
            StellarObjectEnum::Spaceship(s) => s.position,
        }
    }

    /// Récupère la vitesse de l'objet en fonction de son type.
    fn get_velocity(&self) -> Vec2 {
        match self {
            StellarObjectEnum::Asteroid(a) => a.velocity,
            StellarObjectEnum::Missile(m) => m.direction * m.speed,
            StellarObjectEnum::Spaceship(s) => s.velocity,
        }
    }

    /// Vérifie si l'objet est actif dans le jeu.
    fn is_active(&self) -> bool {
        match self {
            StellarObjectEnum::Asteroid(a) => a.is_active(),
            StellarObjectEnum::Missile(m) => m.is_active(),
            StellarObjectEnum::Spaceship(s) => s.shield > 0.0,
        }
    }

    /// Définit l'état actif ou inactif de l'objet.
    fn set_active(&mut self, active: bool) {
        match self {
            StellarObjectEnum::Asteroid(a) => a.set_active(active),
            StellarObjectEnum::Missile(m) => m.active = active,
            StellarObjectEnum::Spaceship(_) => {}, // Le vaisseau est lié à son bouclier.
        }
    }

    /// Met à jour l'objet en fonction de son type.
    ///
    /// # Arguments
    /// * `delta_time` - Le temps écoulé depuis la dernière mise à jour.
    fn update(&mut self, delta_time: f32) {
        match self {
            StellarObjectEnum::Asteroid(a) => a.update(delta_time),
            StellarObjectEnum::Missile(m) => m.update(delta_time),
            StellarObjectEnum::Spaceship(s) => s.update(),
        }
    }

    /// Dessine l'objet à l'écran en fonction de son type.
    fn draw(&self) {
        match self {
            StellarObjectEnum::Asteroid(a) => a.draw(),
            StellarObjectEnum::Missile(m) => m.draw(),
            StellarObjectEnum::Spaceship(s) => s.draw(),
        }
    }

    /// Récupère le rayon de collision de l'objet en fonction de son type.
    fn get_radius(&self) -> f32 {
        match self {
            StellarObjectEnum::Asteroid(a) => match a.size {
                AsteroidSize::Large => 30.0,
                AsteroidSize::Medium => 20.0,
                AsteroidSize::Small => 10.0,
            },
            StellarObjectEnum::Missile(_) => 3.5,
            StellarObjectEnum::Spaceship(_) => 32.0,
        }
    }

    /// Gère la collision entre cet objet et un autre.
    fn handle_collision(&mut self, other: &mut StellarObjectEnum) {
        match (self, other) {
            (Self::Spaceship(ship), Self::Asteroid(asteroid)) => {
                // Check collision with the spaceship and asteroid
                if ship.check_collision_with_asteroid(&vec![asteroid.clone()]) {
                    ship.set_active(false); // Deactivate spaceship if collision leads to game over
                }
            },
            (Self::Asteroid(asteroid), Self::Spaceship(ship)) => {
                // Check collision with the asteroid and spaceship
                if ship.check_collision_with_asteroid(&vec![asteroid.clone()]) {
                    ship.set_active(false);
                }
            },
            (Self::Missile(missile), Self::Asteroid(asteroid)) => {
                // Handle missile and asteroid collision
                if missile.collides_with_asteroid(asteroid) {
                    missile.set_active(false);
                    asteroid.set_active(false);
                }
            },
            (Self::Asteroid(asteroid), Self::Missile(missile)) => {
                // Handle asteroid and missile collision (same as above)
                if missile.collides_with_asteroid(asteroid) {
                    missile.set_active(false);
                    asteroid.set_active(false);
                }
            },
            _ => {}
        }
    }
}

impl StellarObjectEnum {
    fn check_collision_with_asteroid(&mut self, asteroids: &Vec<Asteroid>) -> bool {
        match self {
            StellarObjectEnum::Spaceship(ship) => {
                ship.check_collision_with_asteroid(asteroids)
            },
            _ => false, // Only the spaceship checks for asteroid collisions
        }
    }
    /// Vérifie une collision entre deux objets stellaires.
    ///
    /// # Arguments
    /// * `other` - Une référence à un autre objet stellaire.
    ///
    /// # Retournes
    /// 
    /// `true` si une collision est détectée, sinon `false`.
    pub fn check_collision(&self, other: &StellarObjectEnum) -> bool {
        let distance = self.get_position().distance(other.get_position());
        distance < (self.get_radius() + other.get_radius())
    }

    /// Méthode pratique pour mettre à jour tous les objets du jeu.
    ///
    /// # Arguments
    /// * `objects` - Une référence mutable à une liste d'objets stellaires.
    /// * `delta_time` - Le temps écoulé depuis la dernière mise à jour.
    pub fn update_all(objects: &mut Vec<StellarObjectEnum>, delta_time: f32) {
        for object in objects.iter_mut() {
            object.update(delta_time);
        }
    }

    /// Méthode pratique pour dessiner tous les objets du jeu.
    ///
    /// # Arguments
    /// * `objects` - Une référence à une liste d'objets stellaires.
    pub fn draw_all(objects: &[StellarObjectEnum]) {
        for object in objects {
            object.draw();
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use crate::asteroid::{Asteroid, AsteroidSize}; 

    #[test]
    fn test_position_and_velocity() {
        let spaceship = Spaceship {
            position: Vec2::new(100.0, 100.0),
            velocity: Vec2::new(2.0, 3.0),
            orientation: 0.0,
            shield: 10.0,
            collision_active: false,
            size: 32.0,
        };

        let stellar_object = StellarObjectEnum::Spaceship(spaceship);
        assert_eq!(stellar_object.get_position(), Vec2::new(100.0, 100.0));
        assert_eq!(stellar_object.get_velocity(), Vec2::new(2.0, 3.0));
    }

    #[test]
    fn test_activation_state() {
        let mut asteroid = Asteroid {
            position: Vec2::new(50.0, 50.0),
            velocity: Vec2::new(0.0, 0.0),
            active: true,
            size: AsteroidSize::Large,
        };

        let mut stellar_object = StellarObjectEnum::Asteroid(asteroid.clone());
        assert!(stellar_object.is_active());
        
        stellar_object.set_active(false);
        assert!(!stellar_object.is_active());
    }

    #[test]
    fn test_collision_check() {
        let asteroid = StellarObjectEnum::Asteroid(Asteroid {
            position: Vec2::new(50.0, 50.0),
            velocity: Vec2::new(0.0, 0.0),
            active: true,
            size: AsteroidSize::Large,
        });

        let missile = StellarObjectEnum::Missile(Missile {
            position: Vec2::new(50.0, 50.0),
            direction: Vec2::new(1.0, 0.0),
            speed: 5.0,
            active: true,
        });

        assert!(missile.check_collision(&asteroid));
    }

    #[test]
    fn test_update_all() {
        let mut spaceship = StellarObjectEnum::Spaceship(Spaceship {
            position: Vec2::new(10.0, 10.0),
            velocity: Vec2::new(1.0, 0.0),
            orientation: 0.0,
            shield: 10.0,
            collision_active: false,
            size: 32.0,
        });

        let delta_time = 1.0;
        StellarObjectEnum::update_all(&mut vec![spaceship], delta_time);
        assert_eq!(spaceship.get_position(), Vec2::new(11.0, 10.0));
    }
}