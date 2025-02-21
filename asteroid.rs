
use std::f32::consts::PI;
use macroquad::prelude::*;
use ::rand::{thread_rng, Rng};
use crate::missile::Missile;


///Les differents tailles des asteroids
#[derive(Clone, Copy)]
pub enum AsteroidSize {
    Large,
    Medium,
    Small,
}

/// La structure des Asteroids dans le jeu
/// cette structure contient les informations sur la position, la forme, le size, 
/// la velocity et si l'asteroid est actif ou pas. les Asteroids se crée soit au hasard
/// ou bien avec des size precis, ils peuvent se divisés en 2 autres asteroids de la taille inferieur
/// lorsqu'ils sont frappés par des missiles.
#[derive(Clone)]
pub struct Asteroid {
    pub position: Vec2,
    pub size: AsteroidSize,
    pub velocity: Vec2,
    pub shape: Vec<Vec2>,
    pub active: bool,
}



impl Asteroid {
    ///le methode new_random génère un nouveau asteroid avec des propriétes aléatoire, 
    /// la forme est créée en utilisant des offsets aleatoirs pour créer une forme 
    /// irrégulière.
    //methode pour cree un nv asteroid avec des propriete random
    pub fn new_random() -> Self {
        let mut rng = thread_rng();
        let size = match rng.gen_range(0..3) {
            0 => AsteroidSize::Large,
            1 => AsteroidSize::Medium,
            _ => AsteroidSize::Small,
        };
        let position = Vec2::new(rng.gen_range(0.0..screen_width()), rng.gen_range(0.0..screen_height()));
        let velocity = Vec2::new(rng.gen_range(-30.0..20.0), rng.gen_range(-30.0..20.0));

        // Générer une forme statique
        let radius = match size {
            AsteroidSize::Large => 30.0,
            AsteroidSize::Medium => 20.0,
            AsteroidSize::Small => 10.0,
        };
        let points = 15; // Numero de points sur la forme 
        let mut shape = Vec::new();
        for i in 0..points {
            let angle = (i as f32 / points as f32) * PI * 2.0;
            let offset_x = radius + rng.gen_range(-5.0..5.0);
            let offset_y = radius + rng.gen_range(-5.0..5.0);
            let x = offset_x * angle.cos();
            let y = offset_y * angle.sin();
            shape.push(Vec2::new(x, y));
        }

        Asteroid { position, size, velocity, shape, active: true }

    }
    
    ///la méthode 'new' génére un asteroid avec une taille spécifique, 
    /// mais une position et vitesse aleatoire.
    //mtd pour cree un astroid avec des propriete specifique
    pub fn new(size: AsteroidSize) -> Self {
        let mut rng = thread_rng();
        let position = Vec2::new(rng.gen_range(0.0..screen_width()), rng.gen_range(0.0..screen_height()));
        let velocity = Vec2::new(rng.gen_range(-30.0..25.0), rng.gen_range(-30.0..25.0)); // Random vitesse
    
        // Generate a static shape based on the size
        let radius = match size {
            AsteroidSize::Large => 30.0,
            AsteroidSize::Medium => 20.0,
            AsteroidSize::Small => 10.0,
        };
    
        let points = 15; // Number of points for the shape
        let mut shape = Vec::new();
        for i in 0..points {
            let angle = (i as f32 / points as f32) * PI * 2.0;
            let offset_x = radius + rng.gen_range(-5.0..5.0);
            let offset_y = radius + rng.gen_range(-5.0..5.0);
            let x = offset_x * angle.cos();
            let y = offset_y * angle.sin();
            shape.push(Vec2::new(x, y));
        }
    
        // Return the new asteroid instance with the generated shape
        Asteroid { position, size, velocity, shape, active : true }
    }
    /// la méthode 'update' modifie la position de l'asteroid par rapport a la vitesse
    /// et gère le screen wrapping, 
    /// #arguments
    /// *delta_time -> le temps ecoulé depuis la dérniere image 
    pub fn update(&mut self, delta_time: f32) {
        //let speed_reduction_factor= 0.5
        self.position += self.velocity * delta_time;

        // Wrap around screen
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


    ///cette methode gére les collisions des missiles avec les asteroids 
    /// l'asteroid se casse en deux asteroids de tailles inferieurs lors d'une collision
    /// si l'asteroid est de taille 'small' il disparait .
    /// 
    /// #arguments
    /// * 'missiles' -> réf mutable à la liste de missiles 
    /// * 'asteroids' -> réf mutable de la liste des nouveau asteroids crée aprés le split. 
    pub fn missiles_collision(&mut self, missiles: &mut Vec<Missile>, asteroids: &mut Vec<Asteroid>) {
        let mut new_asteroids = Vec::new();

    // Iterate over missiles to detect collisions
        for missile in missiles.iter_mut() {
            if missile.collides_with_asteroid(self) {
            // Mark the current asteroid as inactive
                self.active = false;

            // Split the asteroid into smaller ones if possible
                if let Some((mut asteroid1, mut asteroid2)) = self.split() {
                // Adjust child asteroid velocities slightly
                    asteroid1.velocity = self.velocity + Vec2::new(5.0, 5.0);
                    asteroid2.velocity = self.velocity - Vec2::new(5.0, 5.0);

                // Add child asteroids to the list of new asteroids
                    new_asteroids.push(asteroid1);
                    new_asteroids.push(asteroid2);
                }

            // Deactivate the missile
                missile.active = false;
            }
    }

    // Add new asteroids to the main list
    asteroids.extend(new_asteroids);

    // Retain only active missiles and asteroids
    missiles.retain(|missile| missile.active);
}


    /// la méthode 'draw' desine les asteroid dans le canvas
    /// elle connecte les points des asteroids on utilisant des lignes
    pub fn draw(&self) {
        for i in 0..self.shape.len() {
            let next_index = (i + 1) % self.shape.len();
            let start = self.position + self.shape[i];
            let end = self.position + self.shape[next_index];
            draw_line(start.x, start.y, end.x, end.y, 2.5, PINK);
        }
    }

    /// la méthode 'split' casse chaque asteroid lors d'une collision avec un missile
    ///  en deux asteroids de taille inferieur, l'asteroid ne se divisera pas si il est 
    /// déja de taille petite.
    pub fn split(&self) -> Option<(Asteroid, Asteroid)> {
        // Déterminer la taille des nouveaux astéroïdes
        let new_size = match self.size {
            AsteroidSize::Large => AsteroidSize::Medium,
            AsteroidSize::Medium => AsteroidSize::Small,
            AsteroidSize::Small => return None, // Pas de division si l'astéroïde est de petite taille
        };

        // Créer deux nouveaux astéroïdes
        let mut asteroid1 = Asteroid::new(new_size);
        let mut asteroid2 = Asteroid::new(new_size);

        // Déplacer les nouveaux astéroïdes pour éviter la superposition exacte
        asteroid1.position = self.position + Vec2::new(5.0, 5.0);  // Déplacement léger pour asteroid1
        asteroid2.position = self.position + Vec2::new(-5.0, -5.0);  // Déplacement léger pour asteroid2

        // Conserver la vitesse du parent pour les nouveaux astéroïdes
        asteroid1.velocity = self.velocity;
        asteroid2.velocity = self.velocity;

        Some((asteroid1, asteroid2)) // Return wrapped in Some
    }

    //pub fn is_active(&self) -> bool {
        //self.active
    //}
    //pub fn set_active(&mut self, active: bool) {
        //self.active= active;
    //}

    pub fn get_position(&self) -> Vec2 {
        self.position
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
    use macroquad::prelude::Vec2;


    #[test]
    fn test_new_random_asteroid() {
        // Vérifie qu'un astéroïde généré aléatoirement est valide
        let asteroid = Asteroid::new_random();
        assert!(asteroid.active);
        assert!(asteroid.position.x >= 0.0 && asteroid.position.x <= screen_width());
        assert!(asteroid.position.y >= 0.0 && asteroid.position.y <= screen_height());
    }

    #[test]
    fn test_update_position() {
        // Vérifie que la position change après un update
        let mut asteroid = Asteroid::new_random();
        let pos_initiale = asteroid.position;
        asteroid.update(1.0);
        assert_ne!(asteroid.position, pos_initiale);
    }

    #[test]
    fn test_screen_wrap() {
        // Vérifie le comportement de wrap autour de l'écran
        let mut asteroid = Asteroid::new_random();
        asteroid.position.x = -10.0;
        asteroid.update(0.0);
        assert_eq!(asteroid.position.x, screen_width());

        asteroid.position.y = screen_height() + 10.0;
        asteroid.update(0.0);
        assert_eq!(asteroid.position.y, 0.0);
    }

    #[test]
    fn test_asteroid_split() {
        // Vérifie la division des astéroïdes
        let asteroid = Asteroid::new(AsteroidSize::Large);
        if let Some((a1, a2)) = asteroid.split() {
            assert!(matches!(a1.size, AsteroidSize::Medium));
            assert!(matches!(a2.size, AsteroidSize::Medium));
        } else {
            panic!("La division d'un grand astéroïde a échoué.");
        }
    }

    #[test]
    fn test_small_asteroid_no_split() {
        // Vérifie qu'un petit astéroïde ne se divise pas
        let asteroid = Asteroid::new(AsteroidSize::Small);
        assert!(asteroid.split().is_none());
    }

    #[test]
    fn test_missile_collision() {
        // Créer un astéroïde de taille grande à une position fixe pour garantir la collision
        let mut asteroid = Asteroid::new(AsteroidSize::Large);
        asteroid.position = Vec2::new(100.0, 100.0); // Position fixe pour la collision garantie

        // Créer un missile à la même position que l'astéroïde pour simuler une collision
        let mut missiles = vec![Missile {
            position: Vec2::new(100.0, 100.0), // Même position pour simuler une collision
            direction: Vec2::new(100.0, 100.0),
            speed: 10.0,
            active: true,
        }];

        // Liste pour stocker les nouveaux astéroïdes créés après la collision
        let mut new_asteroids = Vec::new();

        // Vérifier les collisions entre les missiles et l'astéroïde
        asteroid.missiles_collision(&mut missiles, &mut new_asteroids);

        // Sortie de débogage
        println!("Astéroïde actif: {}", asteroid.active);
        println!("Nombre de missiles après la collision: {}", missiles.len());
        println!("Nouveaux astéroïdes créés: {}", new_asteroids.len());

        // Assertions
        assert!(!asteroid.active, "L'astéroïde devrait être inactif après la collision");
    
        // Vérifier si le vecteur de missiles est vide ou si le premier missile est inactif
        if !missiles.is_empty() {
            assert!(!missiles[0].active, "Le missile devrait être inactif après la collision");
        } else {
            assert!(true, "Le vecteur de missiles est vide comme prévu après la collision");
        }

        // Vérifier si deux nouveaux astéroïdes ont été créés
        assert_eq!(new_asteroids.len(), 2, "Deux nouveaux astéroïdes devraient être créés après une division");

    }

}
