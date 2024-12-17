use std::collections::HashMap;

use kiss3d::light::Light;
use kiss3d::nalgebra::Translation3;
use kiss3d::scene::SceneNode;
use kiss3d::window::{State, Window};
use rapier3d::prelude::*;

struct AppState {
    spheres: HashMap<RigidBodyHandle, SceneNode>,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    integration_parameters: IntegrationParameters,
    island_manager: IslandManager,
    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
    physics_pipeline: PhysicsPipeline,
}

impl AppState {
    pub fn new(window: &mut Window) -> Self {
        window.set_light(Light::StickToCamera);

        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        {
            let table = [
                (vector![0.0, 50.0, 0.0], (50.0, 0.1, 50.0)),
                (vector![0.0, -50.0, 0.0], (50.0, 0.1, 50.0)),
                (vector![50.0, 0.0, 0.0], (0.1, 50.0, 50.0)),
                (vector![-50.0, 0.0, 0.0], (0.1, 50.0, 50.0)),
                (vector![0.0, 0.0, 50.0], (50.0, 50.0, 0.1)),
                (vector![0.0, 0.0, -50.0], (50.0, 50.0, 0.1)),
            ];

            for (translation, size) in table {
                let rigid_body = RigidBodyBuilder::fixed().translation(translation).build();
                let floor_body_handle = rigid_body_set.insert(rigid_body);

                let floor = ColliderBuilder::cuboid(size.0, size.1, size.2)
                    .restitution(1.0)
                    // .friction(1.0)
                    .build();
                collider_set.insert_with_parent(floor, floor_body_handle, &mut rigid_body_set);
            }
        }

        let mut spheres = HashMap::default();
        for index in 0..2000 {
            let sphere = ColliderBuilder::ball(0.5)
                .restitution(1.0)
                // .friction(1.0)
                .build();
            let sphere_body = RigidBodyBuilder::dynamic()
                .translation(vector![(index % 50) as f32, 0.5 + (index / 50) as f32, 0.0])
                .linvel(3.0 * vector![(index % 3) as f32, (index % 5) as f32, (index % 6) as f32])
                .build();
            // sphere_body.add_torque(vector![0.0, 0.1, 0.0], true);
            let sphere_body_handle = rigid_body_set.insert(sphere_body);

            collider_set.insert_with_parent(
                sphere.clone(),
                sphere_body_handle,
                &mut rigid_body_set,
            );

            let mut c = window.add_sphere(0.5);
            c.set_color(1.0, 0.0, 0.0);
            spheres.insert(sphere_body_handle, c);
        }

        let integration_parameters = IntegrationParameters::default();
        let island_manager = IslandManager::new();
        let broad_phase = DefaultBroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();
        let query_pipeline = QueryPipeline::new();
        let physics_pipeline = PhysicsPipeline::new();

        Self {
            spheres,
            rigid_body_set,
            collider_set,
            integration_parameters,
            island_manager,
            broad_phase,
            narrow_phase,
            impulse_joint_set,
            multibody_joint_set,
            ccd_solver,
            query_pipeline,
            physics_pipeline,
        }
    }
}

impl State for AppState {
    fn step(&mut self, _window: &mut Window) {
        let gravity = vector![0.0, 0.0, 0.0];
        self.physics_pipeline.step(
            &gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &(),
            &(),
        );

        for (handle, node) in &mut self.spheres {
            let righd_body = &self.rigid_body_set[*handle];
            let t = righd_body.translation();

            let translation = Translation3::new(t.x, t.y, t.z);
            node.set_local_translation(translation);
        }
    }
}

fn main() {
    let mut window = Window::new("Kiss3d: wasm example");

    let state = AppState::new(&mut window);
    window.render_loop(state)
}
