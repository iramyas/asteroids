use macroquad::prelude::*;
use crate::asteroid::Asteroid;
use crate::spaceship::Spaceship;
use crate::missile::Missile;
use ::rand::{thread_rng, Rng};


mod asteroid;
mod spaceship;
mod missile;
//mod stellarobject;

// gerer les mouvement du vaisseau et les tires des missiles
fn handle_inputs(spaceship: &mut Spaceship, missiles: &mut Vec<Missile>) {
    if is_key_down(KeyCode::Up) {
        spaceship.thrust();
    }
    if is_key_down(KeyCode::Down) {
        spaceship.retro_thrust();
    }
    if is_key_down(KeyCode::Right) {
        spaceship.rotate(1.75);
    }
    if is_key_down(KeyCode::Left) {
        spaceship.rotate(-1.75);
    }
    if is_key_pressed(KeyCode::Space) {
        let direction = Vec2::new(
            spaceship.orientation.to_radians().cos(),
            spaceship.orientation.to_radians().sin(),
        );
        let missile = Missile::new(spaceship.position, direction);
        missiles.push(missile);
    }
}

// Initialisation des étoiles 
fn initialize_stars(stars: &mut Vec<Vec2>, count: usize) {
    stars.clear();
    let mut rng = thread_rng();
    for _ in 0..count {
        let x = rng.gen_range(0.0..screen_width());
        let y = rng.gen_range(0.0..screen_height());
        stars.push(Vec2::new(x, y));
    }
}

//dessin d'etoiles 
fn draw_stars(stars: &Vec<Vec2>) {
    for star in stars {
        draw_circle(star.x, star.y, 1.0, WHITE);
    }
}

// verifie si la souris est sur le restart botton 
fn mouse_over_restart_button() -> bool {
    let button_x = screen_width() / 2.0 - 55.0;
    let button_y = screen_height() / 2.0 + 50.0;
    let mouse_pos = mouse_position();
    mouse_pos.0 >= button_x && mouse_pos.0 <= button_x + 160.0 &&
    mouse_pos.1 >= button_y && mouse_pos.1 <= button_y + 40.0
}

//dessin 'play again?' bouton 
fn draw_restart_button() {
    let button_x = screen_width() / 2.0 - 55.0;
    let button_y = screen_height() / 2.0 + 50.0;
    draw_rectangle(button_x, button_y, 130.0, 30.0, LIGHTGRAY);
    draw_text("play again?", button_x + 15.0, button_y + 20.0, 20.0, PINK);
}

#[macroquad::main("Asteroids")]
async fn main() {
    let mut stars = Vec::new();
    let mut last_width = screen_width();
    let mut last_height = screen_height();
    initialize_stars(&mut stars, 100);
    
    let asteroid_count = 5;
    let mut asteroids: Vec<Asteroid> = (0..asteroid_count).map(|_| Asteroid::new_random()).collect();
    let mut missiles: Vec<Missile> = Vec::new();
    let mut ship = Spaceship::new().await;
    
    let mut game_over = false;
    let mut you_win = false;

    loop {
        clear_background(BLACK);
        draw_stars(&stars);

        // Handle game over or win
        if game_over || you_win {
            if game_over {
                draw_text("LOSER!", screen_width() / 2.0 - 30.0, screen_height() / 2.0 - 20.0, 30.0, ORANGE);
            } else if you_win {
                draw_text("YAY YOU WIN!", screen_width() / 2.0 - 70.0, screen_height() / 2.0 - 20.0, 30.0, PINK);
            }
            draw_restart_button();

            if is_mouse_button_pressed(MouseButton::Left) && mouse_over_restart_button() {
                ship = Spaceship::new().await;
                asteroids = (0..asteroid_count).map(|_| Asteroid::new_random()).collect();
                missiles.clear();
                game_over = false;
                you_win = false;
            }
            next_frame().await;
            continue;
        }

        // Handle inputs et mise a jour du vaisseau  
        handle_inputs(&mut ship, &mut missiles);
        ship.update();
        ship.draw();

        // reinitialisation des etoides en cas d'expension d'écran 
        let current_width = screen_width();
        let current_height = screen_height();
        if current_width != last_width || current_height != last_height {
            initialize_stars(&mut stars, 100);
            last_width = current_width;
            last_height = current_height;
        }

        // Handle d'asteroids
        let mut new_asteroids = Vec::new();
        for asteroid in asteroids.iter_mut() {
            asteroid.update(get_frame_time());
            asteroid.missiles_collision(&mut missiles, &mut new_asteroids);
            asteroid.draw();
        }
        asteroids.extend(new_asteroids);

        // la supression/remove des missiles et asteroid inactifs
        missiles.retain(|missile| missile.active);
        asteroids.retain(|asteroid| asteroid.active);

        
        //verifier les collisions avec le vaiseau
        if ship.check_collision_with_asteroid(&asteroids) && ship.shield <=0.0 {
                game_over = true;
        }

        // Handle missiles
        for missile in &mut missiles {
            missile.update(get_frame_time());
            missile.draw();
        }

        // Display remaining life
        draw_text(
            &format!("life remaining: {}", ship.shield),
            screen_width() - 350.0,
            20.0,
            30.0,
            GREEN,
        );

        // Check win condition
        if asteroids.is_empty() {
            you_win = true;
        }

        next_frame().await;
    }
}
