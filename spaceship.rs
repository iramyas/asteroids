use macroquad::prelude::*; 
use crate::asteroid::{Asteroid, AsteroidSize};
use std::f32::consts::PI;


/// Représente un vaisseau spatial dans le jeu.
///
/// Le vaisseau spatial est contrôlé par le joueur et dispose de capacités de 
/// mouvement, de rotation, et d'un bouclier protecteur. Il peut également 
/// interagir avec d'autres objets dans le jeu, comme les astéroïdes.
#[derive(Clone)]
pub struct Spaceship {
    pub position: Vec2,       // Position actuelle du vaisseau sur l'écran.
    pub orientation: f32,     // Orientation du vaisseau en degrés.
    pub shield: f32,          // Points de bouclier du vaisseau.
    pub collision_active: bool,   // Indique si le vaisseau est actuellement en collision.
    pub velocity: Vec2,       // Vitesse du vaisseau (direction et amplitude).
    pub size: f32,                // Taille du vaisseau, utilisée pour les calculs de collision et de dessin.
}


impl Spaceship {
    /// Crée une nouvelle instance de `Spaceship` avec des valeurs initiales.
    /// 
    /// Le vaisseau est initialisé au centre de l'écran avec une orientation neutre 
    /// et une vitesse nulle. Son bouclier commence avec une force de 10 unités.
    /// 
    /// # Retourne
    /// Une instance de `Spaceship`.
    pub async fn new() -> Self {
        Spaceship {
            position: Vec2::new(screen_width() / 2.0, screen_height() / 2.0),
            orientation: 0.0,
            //texture,
            velocity: Vec2::new(0.0, 0.0),
            shield: 5.0 ,    //10.0force initial du shield 
            collision_active: false,
            size: 32.0, 
        }
    }

    /// Fait avancer le vaisseau dans la direction indiquée par son orientation.
    /// 
    /// La méthode ajuste la vélocité du vaisseau en fonction de son angle actuel,
    /// permettant de simuler un effet de poussée (thrust). Si le vaisseau atteint
    /// un bord de l'écran, il réapparaît de l'autre côté grâce à `wrap_around`.
    pub fn thrust(&mut self) {
        let direction = Vec2::new(
            self.orientation.to_radians().cos(),
            self.orientation.to_radians().sin(),
        );
        self.velocity += direction * 0.1; 
        self.wrap_around();
    }

    /// Fait reculer le vaisseau en inversant la poussée (thrust rétro).
    ///
    /// Semblable à `thrust`, mais réduit la vélocité dans la direction opposée
    /// à l'orientation actuelle du vaisseau.
    pub fn retro_thrust(&mut self) {
        
        let direction = Vec2::new(
            self.orientation.to_radians().cos(),
            self.orientation.to_radians().sin(),
        );
        self.velocity -= direction * 0.1;

        self.wrap_around();
    }

    /// Gère le passage du vaisseau d'un bord de l'écran à l'autre.
    ///
    /// Cette méthode est appelée chaque fois que le vaisseau dépasse les limites
    /// de l'écran, pour éviter qu'il disparaisse. Si le vaisseau sort d'un bord,
    /// il réapparaît automatiquement du côté opposé.
    fn wrap_around(&mut self) {
        if self.position.x < 0.0 {
            self.position.x = screen_width();
        } else if self.position.x > screen_width() {
            self.position.x = 0.0;
        }
    
        if self.position.y < 0.0 {
            self.position.y = screen_height();
        } else if self.position.y > screen_height() {
            self.position.y = 0.0;
        }
    }
    
    /// Met à jour la position du vaisseau en fonction de sa vélocité.
    ///
    /// Simule le mouvement du vaisseau en ajustant sa position. Un effet de 
    /// ralentissement progressif est appliqué en réduisant légèrement la vélocité
    /// à chaque mise à jour.
    pub fn update(&mut self) {
        self.position += self.velocity;
        self.wrap_around();
        self.velocity *= 0.99; // Ralentissement progressif (effet d'inertie).
    }

    /// Fait tourner le vaisseau autour de son axe.
    ///
    /// # Arguments
    /// * `angle` - L'angle de rotation en degrés. Une valeur positive tourne le
    ///   vaisseau vers la gauche, une valeur négative vers la droite.
    pub fn rotate(&mut self, angle: f32) {
        self.orientation = (self.orientation + angle).rem_euclid(360.0);
    }
    
    /// Dessine le vaisseau à l'écran.
    ///
    /// Le vaisseau est représenté par un triangle orienté selon son angle actuel.
    /// La méthode calcule les sommets du triangle et les dessine avec la couleur `DARKPURPLE`.
    pub fn draw(&self) {
        let angle_rad = self.orientation.to_radians();
        let tip_length = self.size * 1.5;
        let base_length = self.size * 0.5;  

        let tip = self.position + Vec2::new(angle_rad.cos(), angle_rad.sin()) * tip_length;
        let left = self.position + Vec2::new((angle_rad + 2.0 * PI / 3.0).cos(), (angle_rad + 2.0 * PI / 3.0).sin()) * base_length;
        let right = self.position + Vec2::new((angle_rad - 2.0 * PI / 3.0).cos(), (angle_rad - 2.0 * PI / 3.0).sin()) * base_length;

        draw_triangle(tip, left, right, DARKPURPLE);
    }

    /* 
    pub fn set_active(&mut self, active: bool) {
        if active {
            self.shield = 5.0; // Reset shield or any other state if activating
            self.collision_active = false; // Reset collision state if activating
        } else {
            self.shield = 0.0; // Deactivate spaceship by setting shield to 0
        }
    }*/

    
    /// Vérifie les collisions entre le vaisseau et une liste d'astéroïdes.
    /// 
    ///la verification se base sur le calcule de distance entre les objets en tenant compte de leurs rayons respectifs.
    /// # Arguments
    /// * `asteroids` - Une référence à un vecteur contenant les astéroïdes à vérifier.
    ///
    /// # Retourne
    /// * `true` si une collision est detecter,
    /// * `false` sinon.
    ///
    /// En cas de collision, le bouclier est réduit d'une unité. Si le bouclier atteint
    /// zéro, la méthode retourne `true` pour signaler la fin de la partie.    
    pub fn check_collision_with_asteroid(&mut self, asteroids: &Vec<Asteroid>) -> bool {
        for asteroid in asteroids {
            let asteroid_position = asteroid.get_position();
            let distance = self.position.distance(asteroid_position);
    
            let asteroid_radius = match asteroid.size {
                AsteroidSize::Large => 30.0,
                AsteroidSize::Medium => 20.0,
                AsteroidSize::Small => 10.0,
            };
            let spaceship_radius = self.size / 2.0;
    
            // verification precise d'une collision
            if distance < (spaceship_radius + asteroid_radius) {
                if !self.collision_active {
                    self.shield -= 1.0;
                    println!("Collision detected! Shield is at: {}", self.shield);
                }
                self.collision_active = true;
                return true;
            }
        }
        
        self.collision_active = false;
        false
    }   

}


#[cfg(test)]
fn screen_width() -> f32 {
    800.0 // Example mock value
}

#[cfg(not(test))]
fn screen_width() -> f32 {
    macroquad::window::screen_width()
}

#[cfg(test)]
fn screen_height() -> f32 {
    600.0 // Example mock value
}

#[cfg(not(test))]
fn screen_height() -> f32 {
    macroquad::window::screen_height()
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::asteroid::{Asteroid, AsteroidSize}; 
    
    
    #[test]
    fn test_initialisation_spaceship() {
        let spaceship = futures::executor::block_on(Spaceship::new());
        assert_eq!(spaceship.position.x, screen_width()/ 2.0);
        assert_eq!(spaceship.position.y, screen_height() /2.0);
        assert_eq!(spaceship.orientation, 0.0);
        assert_eq!(spaceship.shield, 5.0); 
        assert_eq!(spaceship.velocity, Vec2::new(0.0 , 0.0)); 
    }

    #[test]
    fn test_thrust() {
        let mut spaceship = futures::executor::block_on(Spaceship::new());
        spaceship.thrust();
        assert!(spaceship.velocity.length() > 0.0); 
    }

    #[test]
    fn test_retro_thrust() {
        let mut spaceship= futures::executor::block_on(Spaceship::new());
        spaceship.thrust();  //on applique le thrust d'abord 
        let initial_velocity=spaceship.velocity;
        spaceship.retro_thrust();
        assert!(spaceship.velocity.length()< initial_velocity.length());
    }
    #[test]
    fn test_spaceship_rotation() {
        let mut spaceship = futures::executor::block_on(Spaceship::new());
        spaceship.rotate(90.0);
        assert_eq!(spaceship.orientation, 90.0);
        spaceship.rotate(270.0);
        assert_eq!(spaceship.orientation, 0.0); // Should wrap around
    }

    #[test]
    fn test_spaceship_update_position() {
        let mut spaceship = futures::executor::block_on(Spaceship::new());
        spaceship.thrust();
        let initial_position = spaceship.position;
        spaceship.update();
        assert!(spaceship.position != initial_position); // Position should change
    }

    #[test]
    fn test_spaceship_wrap_around() {
        let mut spaceship = futures::executor::block_on(Spaceship::new());
        spaceship.position.x = -10.0; // Move out of bounds
        spaceship.wrap_around();
        assert_eq!(spaceship.position.x, screen_width());
    }
    #[test]
    fn test_ship_check_collision() {
    // Scenario 1: Direct Collision
        let mut spaceship = futures::executor::block_on(Spaceship::new());
        spaceship.shield = 5.0;
    
        let mut asteroid = Asteroid::new(AsteroidSize::Small);
        asteroid.position = spaceship.position; // Direct collision
        let asteroids = vec![asteroid];

        assert!(spaceship.check_collision_with_asteroid(&asteroids), 
            "Direct collision should be detected");
        assert!(spaceship.shield < 5.0, 
            "Shield should be reduced after collision");

    // Scenario 2: No Collision
        let mut spaceship = futures::executor::block_on(Spaceship::new());
        spaceship.shield = 5.0;
    
        let asteroid = Asteroid::new(AsteroidSize::Small);
        let far_asteroids = vec![asteroid];
    
        assert!(!spaceship.check_collision_with_asteroid(&far_asteroids), 
            "No collision should be detected when asteroid is far");
        assert_eq!(spaceship.shield, 5.0, 
            "Shield should not be reduced without collision");
    }   

}

