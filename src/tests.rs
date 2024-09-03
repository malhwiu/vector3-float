use super::*;

#[test]
fn default_values() {
    let zero = Vector3::default();
    assert_eq!(zero, Vector3::new(0f64, 0f64, 0f64));
}

#[test]
fn to_bytes_and_back() {
    let vector_a = Vector3 {
        x: 4.0,
        y: 4.0,
        z: 4.0
    };

    assert_eq!(vec![64, 16, 0, 0, 0, 0, 0, 0, 64, 16, 0, 0, 0, 0, 0, 0, 64, 16, 0, 0, 0, 0, 0, 0], vector_a.to_be_bytes().to_vec());

    let bytes = vector_a.to_be_bytes();
    let vector_b = Vector3::from_be_bytes(bytes);
    assert_eq!(vector_a, vector_b.unwrap());
}

#[test]
fn project() {
    let vector_a = Vector3 {
        x: 4.0,
        y: 4.0,
        z: 0.0
    };
    let vector_b = Vector3 {
        x: 1.0,
        y: 2.0,
        z: 0.0
    };
    
    assert_eq!(vector_a.project(&vector_b), Vector3 { x: 2.4, y: 4.8, z: 0.0});
}

#[test]
fn scalar_multiply() {
    let vec = Vector3 {
        x: 1.5, y: -4.3, z: 2.7
    };

    let val: f64 = 5.0;

    assert_eq!(Vector3 {x: 1.5 * val, y: -4.3 * val, z: 2.7 * val}, vec * val);
}

#[test]
fn normalize() {
    let vec = Vector3::new(5.0, 0.0, 0.0);

    assert_eq!(vec.normalize(), Vector3::new(1.0, 0.0, 0.0));
}

#[test]
fn vector_normalization() {
    let v = Vector3 {x: 10.0, y: 5.0, z: 0.0};

    assert_eq!(Vector3 {x: 0.8944271909999159, y: 0.4472135954999579, z: 0.0}, v.normalize());
    assert_eq!(1.0, v.normalize().magnitude().round());
}
#[test]
fn sub_two_vectors() {
    let vector1 = Vector3 {
        x: 1.5, y: -4.3, z: 2.7
    };
    let vector2 = Vector3 {
        x: 1.5, y: -4.3, z: 2.8
    };

    assert_eq!(vector1 - vector2, Vector3 {x: 0.0, y: 0.0, z: -0.09999999999999964});
}
#[test]
fn add_two_vectors() {
    let vector1 = Vector3 {
        x: 1.5, y: -4.3, z: 2.7
    };
    let vector2 = Vector3 {
        x: 1.5, y: -4.3, z: 2.8
    };

    assert_eq!(vector1 + vector2, Vector3 {x: 1.5 * 2.0, y: -4.3 * 2.0, z: 5.5});
}
#[test]
fn vector_magnitude() {
    let vector = Vector3 {
        x: 1.5, y: -4.3, z: 10.1
    };

    assert_eq!(vector.magnitude(), 11.07925990308017);
    assert_eq!(vector.sqrt_magnitude(), 122.74999999999999);
}
#[test]
fn dot_vectors() {
    let vector1 = Vector3 {
        x: 1.5, y: -4.3, z: 2.7
    };
    let vector2 = Vector3 {
        x: 1.5, y: -4.3, z: 2.8
    };

    assert_eq!(vector1 * vector2, Vector3::new(2.25, 18.49, 7.56));
}

#[test]
fn angle_between() {
    let vector_a = Vector3 {
        x: 3.0, y: -2.0, z: 0.0
    };
    let vector_b = Vector3 {
        x: 1.0, y: 7.0, z: 0.0
    };

    assert_eq!(115.55996517182382, vector_a.angle_degrees(&vector_b));

    let vector_a = Vector3 {
        x: 1.0, y: 0.0, z: 0.0
    };

    let vector_b = Vector3 {
        x: 0.0, y: 1.0, z: 0.0
    };

    assert_eq!(vector_a.angle_degrees(&vector_b), 90.0);
    assert_eq!(vector_a.angle_radians(&vector_b), 1.5707963267948966);
}

#[test]
fn cross_product() {
    let vector_a = Vector3::new(1.0, 2.0, 3.0);
    let vector_b = Vector3::new(2.0, 1.0, 3.0);

    assert_eq!(vector_a.cross(&vector_b), Vector3::new(3.0, 3.0, -3.0));
}

#[test]
fn floor_it() {
    let vector = Vector3::new(5.3, 2.1, 5.4);

    assert_eq!(vector.floor(), Vector3::new(5.0, 2.0, 5.0));
}

#[test]
fn ceil_it() {
    let vector = Vector3::new(5.7, 2.6, 5.5);

    assert_eq!(vector.ceil(), Vector3::new(6.0, 3.0, 6.0));
}