use nalgebra::Vector3;

struct Shape {
    vecs: Vec<Vector3<f64>>,
}

impl Shape {

    fn centroid(&self) -> Vector3<f64> {
        let mut center = Vector3::<f64>::new(0.0 as f64, 0.0 as f64, 0.0 as f64);

        for vec in &self.vecs {
            center += vec;
        }
        return center/(self.vecs.len() as f64);
    }
        fn furthest_point(&self, d: Vector3<f64>) -> Vector3<f64> {
            let mut dot_max = 0.0;
            let mut current_vector = self.vecs[0];
           for vec in &self.vecs {
                let dot_current = (vec - self.centroid()).dot(&d);
                if dot_current > dot_max {
                    dot_max = dot_current;
                    current_vector = *vec;
                }
            }
            return current_vector;
        }
        
}

fn support(s1: &Shape, s2: &Shape, d: Vector3<f64>) -> Vector3<f64> {
    return s1.furthest_point(d) - s2.furthest_point(-d);
}

fn trip_prod(v1: Vector3<f64>,v2: Vector3<f64>, v3: Vector3<f64>) -> Vector3<f64> {
    return (v1.cross(&v2)).cross(&v3);
}

fn line_case(simplex: &Vec<Vector3<f64>>, d: &mut Vector3<f64>) -> bool {
    let b = simplex[0];
    let a = simplex[1];
    let ab = b - a;
    let ao = -a;
    let abperp = trip_prod(ab, ao, ab);
    *d = abperp;
    return false;
}

fn triangle_case(simplex: &mut Vec<Vector3<f64>>, d: &mut Vector3<f64>) -> bool {
    let c = simplex[simplex.len() - 3];
    let b = simplex[simplex.len() - 2];
    let a = simplex[simplex.len() - 1];
    let ab = b - a;
    let ac = c - a;
    let ao = -a;
    let abperp = trip_prod(ac, ab, ab);
    let acperp = trip_prod(ab, ac, ac);

    if abperp.dot(&ao) > 0.0 {
        simplex.remove(simplex.len() - 3);
        *d = abperp;
        return false;
    }
    else if acperp.dot(&ao) > 0.0 {
        simplex.remove(simplex.len() - 2);
        *d = acperp;
        return false;
    }
    else {
        return true;
    }
}

fn handle_simplex(simplex: &mut Vec<Vector3<f64>>, d: &mut Vector3<f64>) -> bool {
    return if simplex.len() == 2 {
        line_case(simplex, d)
    } else {
        triangle_case(simplex, d)
    }
}

fn gjk(s1: &Shape, s2: &Shape) -> bool {
    let mut d = (s2.centroid() - s1.centroid()).normalize();
    let mut simplex: Vec<Vector3<f64>> = vec![support(s1, s2, d)];
    d = -simplex[0];

    loop {
        let a = support(s1,s2, d);
        if a.dot(&d) < 0.0 {
            return false;
        }
        else {
            simplex.push(a);
            if handle_simplex(&mut simplex, &mut d) {
                return true;
            }
        }
    }
}

fn main() {


    let s1 = Shape {
        vecs: vec![Vector3::<f64>::new(0.0,0.0,0.0),
                   Vector3::<f64>::new(0.0,1.0,0.0),
                   Vector3::<f64>::new(1.0,0.0,0.0)],
    };

    let s2 = Shape {
        vecs: vec![Vector3::<f64>::new(0.0 + 1.01,0.0,0.0),
                   Vector3::<f64>::new(0.0 + 1.01,1.0,0.0),
                   Vector3::<f64>::new(1.0 + 1.01,0.0,0.0)],
    };

    println!("{}", gjk(&s1, &s2));

}