extern crate tsp_solver;

/*
5
0 0
0 0.5
0 1
1 1
1 0
*/

#[test]
fn construct_a_path() {
    assert_eq!(tsp_solver::a(), 2);
}
