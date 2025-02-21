use crate::asteroid::{Asteroid, AsteroidSize};
use macroquad::prelude::*;


///Represente un missile dans le jeu 
/// 
pub struct Missile {
    pub position: Vec2,
    pub direction: Vec2,
    pub speed: f32,
    pub active: bool,
}


impl Missile {
    ///Crée un nouveau missile avec une position et une direction données.
    /// 
    /// # Arguments
    /// * 'position' -> la position initiale du missile; 
    /// * 'direcvtion' -> la direction dans laquelle le missile se deplace, 
    /// 
    /// #Retourne
    /// Une instance de 'Missile'. 
    pub fn new(position: Vec2, direction: Vec2) -> Self {
        Missile {
            position,
            direction: direction.normalize(), // Direction 
            speed: 450.0,                     // Vitesse du missile
            active: true,
        }
    }

    /// Cette methode met à jour la position du missil en fonction du temps écoulé.
    /// 
    /// #Arguments
    /// * 'dt' -> le écoulé depuis la derniere mise a jour. 
    pub fn update(&mut self, dt: f32) {
        //on utilise les mock valeurs de height et width pour les tests unitaires 
        let screen_width= if cfg!(test) {800.0}else {macroquad::window::screen_width()};
        let screen_height=if cfg!(test){600.0} else {macroquad::window::screen_height()};
        // Mettre à jour la position
        if self.active {
            self.position += self.direction * self.speed * dt;

            // Désactiver le missile s'il sort de l'écran
            if self.position.x < 0.0
                || self.position.x > screen_width
                || self.position.y < 0.0
                || self.position.y > screen_height
            {
                self.active = false;
            }
        }
    }
    

    //pub fn is_active(&self) -> bool {
        //self.active
    //}

    //pub fn set_active(&mut self, active: bool) {
        //self.active = active;
    //}

    /// methode pour dissiner le missile sur l'écran si celui- ci est actif. 
    pub fn draw(&self) {
        if self.active {
            draw_circle(self.position.x, self.position.y, 3.5, BLUE);
        }
    }

    /// La methode 'collides_with' vérifie si le missile rentre en collision avec un astéroid.
    /// 
    /// #Arguments
    /// * 'asteroid' -> une réference à l'asteroide à vérifier
    /// # retourne 
    /// 'true' si le missile rentre en collision avec un asteroid, sinon 'false'. 
    pub fn collides_with_asteroid(&self, asteroid: &Asteroid) -> bool {
        let distance = self.position.distance(asteroid.get_position());
        let asteroid_radius = match asteroid.size {
            AsteroidSize::Large => 30.0,
            AsteroidSize::Medium => 20.0,
            AsteroidSize::Small => 10.0,
        };
        distance < asteroid_radius
    }
}


#[cfg(test)]
mod tests {
    use super::*; 
    use crate::asteroid::{Asteroid, AsteroidSize}; 

    #[test]
    fn test_missile_creation() {
        let position = Vec2::new(100.0, 200.0);
        let direction = Vec2::new(1.0, 0.0);
        let missile= Missile::new(position, direction); 

        assert_eq!(missile.position, position);
        assert_eq!(missile.direction, direction.normalize());
        assert_eq!(missile.speed, 450.0); 
        assert!(missile.active);

    }



    #[test]
    fn test_missile_update() {
        let mut missile = Missile::new(Vec2::new(50.0, 50.0), Vec2::new(1.0, 0.0));
        let dt = 1.0 / 60.0; // Simulating a frame in a 60 FPS game

        missile.update(dt);

        // Use a small tolerance for floating point comparison
        let expected_position = Vec2::new(57.5, 50.0);
        let tolerance = 0.001; // Tolerance for floating point comparison

        assert!((missile.position.x - expected_position.x).abs() < tolerance);
        assert!((missile.position.y - expected_position.y).abs() < tolerance);
    }

    

    #[test]
    fn test_collision_with_asteroid() {
        let asteroid_shape = vec![
            Vec2::new(-10.0, -10.0),
            Vec2::new(10.0, -10.0),
            Vec2::new(10.0, 10.0),
            Vec2::new(-10.0, 10.0),
        ];

        // Asteroid setup
        let asteroid = Asteroid {
            position: Vec2::new(50.0, 50.0),
            size: AsteroidSize::Large,
            velocity: Vec2::new(0.0, 0.0),
            shape: asteroid_shape,
            active: true,
        };

        // Missile setup: collision case
        let missile_in_collision = Missile {
            position: Vec2::new(50.0, 50.0),
            direction: Vec2::new(1.0, 0.0),
            speed: 5.0,
            active: true,
        };

        // Missile setup: no collision case
        let missile_out_of_collision = Missile {
            position: Vec2::new(100.0, 100.0),
            direction: Vec2::new(-1.0, -1.0),
            speed: 5.0,
            active: true,
    };

    // Assert collision detected
    assert!(missile_in_collision.collides_with_asteroid(&asteroid));

    // Assert no collision detected
    assert!(!missile_out_of_collision.collides_with_asteroid(&asteroid));
}

}

/* 

#[test]
    fn test_missile_out_of_bounds() {
        let mut missile = Missile::new(Vec2::new(10.0, 10.0), Vec2::new(-1.0, 0.0));

        missile.update(1.0); // bouger le missile pendant une seconde 
        assert!(!missile.is_active());
    }*/