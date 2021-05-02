use nalgebra::Vector3;


struct Shape {
    vecs: Vec<Vector3<f32>>,
}

impl Shape {

    fn centroid(&mut self) -> Vector3<f32> {
        let mut center = self.vecs[0];
        for i in 1..self.vecs.len() {
            center += self.vecs[i];
        }
        return center;
    }

    fn init(&mut self, points: Vec<Vector3<f32>>) {
        for point in points {
            self.vecs.push(point);
        }
    }

    fn furthestPoint(&mut self, d: &mut Vector3<f32>) -> Vector3<f32> {
        let mut dotMax = 0.0;
        let mut currentVector = Vector3::<f32>::new(0.0,0.0,0.0);
        for i in 0..self.vecs.len() {
            let mut vec = self.vecs[i];
            let mut dotCurrent = (vec - self.centroid()).dot(&d);
            if dotCurrent > dotMax {
                dotCurrent = dotMax;
                currentVector = vec;
            }
        }
        return currentVector;
    }
}

fn support(s1: &mut Shape, s2: &mut Shape, d: &mut Vector3<f32>) -> Vector3<f32> {
    let mut reverse_d= *d;
    reverse_d.x = -reverse_d.x;
    reverse_d.y = -reverse_d.y;
    reverse_d.z = -reverse_d.z;
    return s1.furthestPoint(d) - s2.furthestPoint(&mut reverse_d);
}

fn tripProd(v1: Vector3<f32>,v2: Vector3<f32>, v3: Vector3<f32>) -> Vector3<f32> {
    return (v1.cross(&v2)).cross(&v3);
}

fn lineCase(simplex: &mut Vec<Vector3<f32>>, d: &mut Vector3<f32>) -> bool{
    let mut B = simplex[0];
    let mut A = simplex[1];
    let mut AB = B - A;
    let mut AO = Vector3::<f32>::new(0.0,0.0,0.0) - A;
    let mut ABPerp = tripProd(AB, AO, AB);
    *d = ABPerp;
    return false;
}

fn main() {
}