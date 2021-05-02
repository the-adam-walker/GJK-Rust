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


fn lineCase(simplex: &mut Vec<Vector3<f32>>, d: &mut Vector3<f32>) -> bool {
    let mut B = simplex[0];
    let mut A = simplex[1];
    let mut AB = B - A;
    let mut AO = Vector3::<f32>::new(0.0, 0.0, 0.0) - A;
    let mut ABPerp = tripProd(AB, AO, AB);
    *d = ABPerp;
    return false;
}

fn triangleCase(simplex: &mut Vec<Vector3<f32>>, d: &mut Vector3<f32>) -> bool {

    let mut C = simplex[simplex.len() - 3];
    let mut B = simplex[simplex.len() - 2];
    let mut A = simplex[simplex.len() - 1];

    let mut AB = B - A;
    let mut AC = C - A;
    let mut AO = -A;

    let mut ABPerp = tripProd(AC, AB, AB);
    let mut ACPerp = tripProd(AB, AC, AC);

    if ABPerp.dot(&AO) > 0.0 {
        simplex.remove(simplex.len() - 3);
        *d = ABPerp;
        return false;
    }
    else if ACPerp.dot(&AO) > 0.0 {
        simplex.remove(simplex.len() - 2);
        *d = ACPerp;
        return false;
    }

    return true;

}

fn handleSimplex(simplex: &mut Vec<Vector3<f32>>, d: &mut Vector3<f32>) -> bool {
    if simplex.len() == 2 {
        return lineCase(simplex, d);
    }
    else {
        return triangleCase(simplex, d);
    }
}


fn GJK(s1: &mut Shape, s2: &mut Shape) -> bool {

    //Vector3<T> d = (s2.centroid() - s1.centroid()).normalize();
    let mut d = (s2.centroid() - s1.centroid()).normalize();

    let mut simplex: Vec<Vector3<f32>>;
    simplex.push(support(s1, s2, &mut d));


    d = Vector3::new(0.0,0.0,0.0) - simplex[0];

    while true {
        let mut A = support(s1, s2, &mut d);
        if A.dot(d) < 0.0 {
            return false;
        }
        else {
            simplex.push(A);
            if handleSimplex(simplex,&mut d) {
                return true;
            }
        }
    }

    return true;
}

fn main() {


}