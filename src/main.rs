use bevy::prelude::*;
use bevy_hanabi::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HanabiPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.01,
            ..default()
        },
        ..default()
    });

    let mut module = Module::default();

    // Just spawn everything in one spot
    let init_pos = SetPositionCircleModifier {
        center: module.lit(Vec3::ZERO),
        axis: module.lit(Vec3::Z),
        radius: module.lit(0.0),
        dimension: ShapeDimension::Surface,
    };

    let prop = module.prop("my_prop");
    let x_vec = module.lit(Vec3::X);
    // let identity_mat = module.lit(MatrixValue::from(Mat3::IDENTITY));

    // Transform the x axis with the my_prop matrix
    let init_x_axis = SetAttributeModifier {
        attribute: Attribute::AXIS_X,
        value: module.mul(prop, x_vec),
        // value: module.mul(identity_mat, x_vec), // This works as you would expect
    };

    let effect = EffectAsset::new(32768, Spawner::rate(10.0.into()), module)
        .with_property("my_prop", Value::Matrix(Mat3::IDENTITY.into())) // Should just be identity matrix
        .init(init_pos)
        .init(init_x_axis);

    commands.spawn(ParticleEffectBundle {
        effect: ParticleEffect::new(effects.add(effect)),
        ..default()
    });
}

#[test]
fn len_check() {
    let value = Value::Matrix(Mat3::IDENTITY.into());
    assert_eq!(value.as_bytes().len(), value.value_type().size());
}
