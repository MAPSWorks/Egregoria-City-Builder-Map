use crate::context::Context;
use egregoria::engine_interaction::{KeyCode, MouseButton};
use geom::{vec2, Camera, Vec2, Vec3};
use wgpu_engine::Tesselator;

pub struct CameraHandler {
    pub camera: Camera,
    pub last_pos: Vec2,
}

const CAMERA_KEY_MOVESPEED: f32 = 300.0;

impl CameraHandler {
    pub fn new(width: f32, height: f32, position: Vec3) -> CameraHandler {
        CameraHandler {
            camera: Camera::new(width, height, position),
            last_pos: vec2(0.0, 0.0),
        }
    }

    pub fn update(&mut self, ctx: &mut Context) {
        self.camera.update();
        ctx.gfx.set_proj(self.camera.projection());
        ctx.gfx.set_inv_proj(self.camera.inv_projection());
    }

    pub fn culled_tesselator(&self) -> Tesselator {
        Tesselator::new(
            Some(self.camera.get_screen_box()),
            1000.0 / self.camera.position.z,
        )
    }

    pub fn resize(&mut self, ctx: &mut Context, width: f32, height: f32) {
        self.camera.set_viewport(width, height);
        self.update(ctx);
    }

    pub fn unproject_mouse_click(&self, pos: Vec2) -> Vec2 {
        self.camera.unproject(pos)
    }

    pub fn easy_camera_movement(
        &mut self,
        ctx: &mut Context,
        delta: f32,
        mouse_enabled: bool,
        keyboard_enabled: bool,
    ) {
        let p = ctx.input.mouse.unprojected;
        if mouse_enabled && ctx.input.mouse.buttons.contains(&MouseButton::Right) {
            self.camera.position.x -= p.x - self.last_pos.x;
            self.camera.position.y -= p.y - self.last_pos.y;
            self.camera.update();
            common::saveload::save_silent(&self.camera, "camera");
        }

        if mouse_enabled {
            self.last_pos = self.unproject_mouse_click(ctx.input.mouse.screen);
            if ctx.input.mouse.wheel_delta < 0.0 {
                self.zoom_by(ctx, 1.1);
            }
            if ctx.input.mouse.wheel_delta > 0.0 {
                self.zoom_by(ctx, 1.0 / 1.1);
            }
        }

        if keyboard_enabled {
            let is_pressed = &ctx.input.keyboard.is_pressed;

            if is_pressed.contains(&KeyCode::Right) {
                self.camera.position.x += delta * CAMERA_KEY_MOVESPEED * self.camera.position.z;
                self.camera.update();
                common::saveload::save_silent(&self.camera, "camera");
            }
            if is_pressed.contains(&KeyCode::Left) {
                self.camera.position.x -= delta * CAMERA_KEY_MOVESPEED * self.camera.position.z;
                self.camera.update();
                common::saveload::save_silent(&self.camera, "camera");
            }
            if is_pressed.contains(&KeyCode::Up) {
                self.camera.position.y += delta * CAMERA_KEY_MOVESPEED * self.camera.position.z;
                self.camera.update();
                common::saveload::save_silent(&self.camera, "camera");
            }
            if is_pressed.contains(&KeyCode::Down) {
                self.camera.position.y -= delta * CAMERA_KEY_MOVESPEED * self.camera.position.z;
                self.camera.update();
                common::saveload::save_silent(&self.camera, "camera");
            }

            let just_pressed = &ctx.input.keyboard.just_pressed;
            if just_pressed.contains(&KeyCode::Add) || just_pressed.contains(&KeyCode::Equals) {
                self.zoom_by(ctx, 1.1);
            }

            let just_pressed = &ctx.input.keyboard.just_pressed; // cannot call zoom_by 2 lines above without reborrowing
            if just_pressed.contains(&KeyCode::Subtract) || just_pressed.contains(&KeyCode::Minus) {
                self.zoom_by(ctx, 1.0 / 1.1);
            }
        }

        self.last_pos = self.unproject_mouse_click(ctx.input.mouse.screen);
    }

    fn zoom_by(&mut self, ctx: &mut Context, multiply: f32) {
        self.camera.position.z *= multiply;

        self.update(ctx);
        let after = self.unproject_mouse_click(ctx.input.mouse.screen);
        self.camera.position.x -= after.x - self.last_pos.x;
        self.camera.position.y -= after.y - self.last_pos.y;
        self.update(ctx);
        common::saveload::save_silent(&self.camera, "camera");
    }
}
