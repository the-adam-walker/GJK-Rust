use nalgebra::Vector3;


struct Shape {
    vecs: Vec<Vector3<f32>>,
}

impl Shape {

    fn init(&mut self, points: Vec<Vector3<f32>>) {
        for point in points {
            self.vecs.push(point);
        }
    }

    fn centroid(&mut self) -> Vector3<f32> {
        let mut center = self.vecs[0];
        for i in 1..self.vecs.len() {
            center += self.vecs[i];
        }
        return center;
    }
}

fn tripProd(v1: Vector3<f32>,v2: Vector3<f32>, v3: Vector3<f32>) -> Vector3<f32> {
    return (v1.cross(v2)).cross(v3);
}


fn main() {

}