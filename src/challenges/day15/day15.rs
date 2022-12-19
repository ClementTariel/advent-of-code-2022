#[path = "../../utils/mod.rs"] mod utils;

pub fn abs(x: i32) -> i32 {
    if x < 0 {
        return -x;
    }
    return x;
}

pub fn part1() {
    let mut bounds: Vec<[i32;2]> = [].to_vec();
    let mut count: i32 = 0;
    let y_ref = 2000000;
    let mut xb_to_remove: Vec<i32> = [].to_vec();
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                let split = line_content.split(": closest beacon is at x=").collect::<Vec<&str>>();
                let sensor_coord = split[0].split("Sensor at x=").collect::<Vec<&str>>()[1].split(", y=").collect::<Vec<&str>>();
                let beacon_coord = split[1].split(", y=").collect::<Vec<&str>>();
                let xs: i32 = sensor_coord[0].parse().unwrap();
                let ys: i32 = sensor_coord[1].parse().unwrap();
                let mut xb: i32 = beacon_coord[0].parse().unwrap();
                let yb: i32 = beacon_coord[1].parse().unwrap();
                let d = abs(ys-yb) + abs(xs-xb);
                if yb == y_ref {
                    let mut xb_already_to_remove = false;
                    for k in 0..xb_to_remove.len(){
                        if xb_to_remove[k] == xb {
                            xb_already_to_remove = true;
                            break;
                        }
                    }
                    if !xb_already_to_remove{
                        xb_to_remove.push(xb);
                    }
                }
                if d >= abs(y_ref-ys){
                    xb = xs - d + abs(y_ref-ys);
                }else{
                    continue;
                }
                let mut intervals: Vec<[i32;2]> = [].to_vec();
                intervals.push([xb, xs + (xs - xb)]);
                let n_i = bounds.len();
                for i in 0..n_i {
                    let n_j = intervals.len();
                    if n_j == 0 {
                        break;
                    }
                    for j  in 0..n_j{
                        if bounds[i][0] <= intervals[n_j-1-j][0] && bounds[i][1] >= intervals[n_j-1-j][1]{
                            intervals.remove(n_j-1-j);
                            continue;
                        }else if bounds[i][0] <= intervals[n_j-1-j][0] && bounds[i][1] >= intervals[n_j-1-j][0]{
                            intervals[n_j-1-j][0] = bounds[i][1] + 1;
                            if intervals[n_j-1-j][0] > intervals[n_j-1-j][1]{
                                intervals.remove(n_j-1-j);
                                continue;
                            }
                        }else if bounds[i][0] <= intervals[n_j-1-j][1] && bounds[i][1] >= intervals[n_j-1-j][1]{
                            intervals[n_j-1-j][1] = bounds[i][0] - 1;
                            if intervals[n_j-1-j][0] > intervals[n_j-1-j][1]{
                                intervals.remove(n_j-1-j);
                                continue;
                            }
                        }else if bounds[i][0] >= intervals[n_j-1-j][0] && bounds[i][1] <= intervals[n_j-1-j][1]{
                            if intervals[n_j-1-j][1] > bounds[i][1]{
                                intervals.push([bounds[i][1] + 1, intervals[n_j-1-j][1]]);
                            }
                            if intervals[n_j-1-j][0] < bounds[i][0]{
                                intervals.push([intervals[n_j-1-j][0], bounds[i][0] - 1]);
                            }
                            intervals.remove(n_j-1-j);
                        }
                    }
                }
                let n_j = intervals.len();
                for j  in 0..n_j{
                    bounds.push(intervals[j]);
                    count += intervals[j][1] - intervals[j][0] + 1;
                }
            }
        }
    }else{
        println!("lines not ok");
    }
    for k in 0..xb_to_remove.len(){
        for i in 0..bounds.len() {
            if (bounds[i][0] <= xb_to_remove[k]) && (bounds[i][1] >= xb_to_remove[k]){
                count -= 1;
            }
        }
    }
    println!("result = {}",count);
}

pub fn truncate_line( mut line:[i32;5],min_val:i32, max_val:i32) -> [i32;5]{
    let mut dir_x = line[2];
    let mut dir_y = line[3];
    let mut d = line[4];
    if dir_x < 0 {
        line = [line[0]+dir_x*d,line[1]+dir_y*d,-dir_x,-dir_y,d];
        dir_x = -dir_x;
        dir_y = -dir_y;
    }
    if line[0] < min_val {
        d -= min_val - line[0];
        line[1] += dir_y*(min_val-line[0]);
        line[0] = min_val;
    }
    if line[0]+d > max_val {
        d = max_val - line[0];
    }
    if dir_y < 0 {
        line = [line[0]+dir_x*d,line[1]+dir_y*d,-dir_x,-dir_y,d];
        dir_x = -dir_x;
        dir_y = -dir_y;
    }
    if line[1] < min_val {
        d -= min_val - line[1];
        line[0] += dir_x*(min_val-line[1]);
        line[1] = min_val;
    }
    if line[1]+d > max_val {
        d = max_val - line[1];
    }
    line[4] = d;
    return line;
}

pub fn check_lines(sensor_coverage:&Vec<[i32;3]>,mut lines:Vec<[i32;5]>,min_val:i32, max_val:i32) -> Vec<[i32;5]>{
    for [sx,sy,sd] in sensor_coverage {
        let n = lines.len();
        for i in 0..n{
            lines[n-1-i] = truncate_line(lines[n-1-i], min_val, max_val);
            let dir_x = lines[n-1-i][2];
            let dir_y = lines[n-1-i][3];
            let mut d = lines[n-1-i][4];
            if d < 0 {
                lines.remove(n-1-i);
                continue;
            }
            let pt1_inside: bool = (abs(lines[n-1-i][0]-sx) + abs(lines[n-1-i][1]-sy)) <= *sd;
            let pt2_inside: bool = (abs(lines[n-1-i][0]+d*lines[n-1-i][2]-sx) + abs(lines[n-1-i][1]+d*lines[n-1-i][3]-sy)) <= *sd;
            let any_pt_inside: bool = (lines[n-1-i][0] < *sx) && (lines[n-1-i][0]+d*lines[n-1-i][2] > *sx) && (abs(lines[n-1-i][1] + dir_y*dir_x*(sx-lines[n-1-i][0]) - sy) <= *sd);
            if pt1_inside && pt2_inside{
                lines.remove(n-1-i);
                continue;
            }
            if pt1_inside {
                let delta_d = d - (abs(lines[n-1-i][0]+d*lines[n-1-i][2]-sx) + abs(lines[n-1-i][1]+d*lines[n-1-i][3]-sy) - sd + 1)/2 + 1;
                lines[n-1-i][0] += dir_x*delta_d;
                lines[n-1-i][1] += dir_y*delta_d;
                d -= delta_d;
                lines[n-1-i][4] = d;
                lines[n-1-i] = truncate_line(lines[n-1-i], min_val, max_val);
                d = lines[n-1-i][4];
                if d < 0 {
                    lines.remove(n-1-i);
                    continue;
                }
            }
            if pt2_inside {
                let delta_d = d - (abs(lines[n-1-i][0]-sx) + abs(lines[n-1-i][1]-sy) - sd + 1)/2 + 1;
                d -= delta_d;
                lines[n-1-i][4] = d;
                lines[n-1-i] = truncate_line(lines[n-1-i], min_val, max_val);
                d = lines[n-1-i][4];
                if d < 0 {
                    lines.remove(n-1-i);
                    continue;
                }
            }
            if (!pt1_inside) && (!pt2_inside) && any_pt_inside {
                let mut d1 = (abs(lines[n-1-i][0]-sx) + abs(lines[n-1-i][1]-sy) - sd + 1)/2 - 1;
                let mut line1:[i32;5] = [lines[n-1-i][0],lines[n-1-i][1],lines[n-1-i][2],lines[n-1-i][3],d1];
                line1 = truncate_line(line1, min_val, max_val);
                d1 = line1[4];
                if d1 >= 0 {
                    lines.push(line1);
                }
                let mut d2 = (abs(lines[n-1-i][0]+d*lines[n-1-i][2]-sx) + abs(lines[n-1-i][1]+d*lines[n-1-i][3]-sy) - sd + 1)/2 - 1;
                let mut line2:[i32;5] = [lines[n-1-i][0]+d*lines[n-1-i][2],lines[n-1-i][1]+d*lines[n-1-i][3],-lines[n-1-i][2],-lines[n-1-i][3],d2];
                line2 = truncate_line(line2, min_val, max_val);
                d2 = line2[4];
                if d2 >= 0 {
                    lines.push(line2);
                }
                lines.remove(n-1-i);
            }
        }
    }
    return lines;
}

pub fn part2() {
    let min_val = 0;
    //let max_val = 20;
    let max_val = 4000000;
    let mut sensor_coverage: Vec<[i32;3]> = [].to_vec();
    let mut lines_potentialy_with_beacon: Vec<[i32;5]> = [].to_vec();
    if let Ok(lines) = utils::lines_from_file("./input.txt"){
        for line in lines {
            if let Ok(line_content) = line {
                // process lines here
                let split = line_content.split(": closest beacon is at x=").collect::<Vec<&str>>();
                let sensor_coord = split[0].split("Sensor at x=").collect::<Vec<&str>>()[1].split(", y=").collect::<Vec<&str>>();
                let beacon_coord = split[1].split(", y=").collect::<Vec<&str>>();
                let xs: i32 = sensor_coord[0].parse().unwrap();
                let ys: i32 = sensor_coord[1].parse().unwrap();
                let xb: i32 = beacon_coord[0].parse().unwrap();
                let yb: i32 = beacon_coord[1].parse().unwrap();
                let d = abs(ys-yb) + abs(xs-xb);
                sensor_coverage.push([xs,ys,d]);
                //println!("{},{},{},{},{}",xs,ys,xb,yb,d);
            }
        }
    }else{
        println!("lines not ok");
    }
    for [sx,sy,d] in &sensor_coverage {
        let line1 = [sx-d-1,*sy,1,1,*d];
        let line2 = [*sx,sy+d+1,1,-1,*d];
        let line3 = [sx+d+1,*sy,-1,-1,*d];
        let line4 = [*sx,sy-d-1,-1,1,*d];
        lines_potentialy_with_beacon.push(line1);
        lines_potentialy_with_beacon.push(line2);
        lines_potentialy_with_beacon.push(line3);
        lines_potentialy_with_beacon.push(line4);
        lines_potentialy_with_beacon = check_lines(&sensor_coverage, lines_potentialy_with_beacon, min_val, max_val);
        if lines_potentialy_with_beacon.len() > 0 {
            let line = lines_potentialy_with_beacon[0];
            println!("{},{} -> {}",line[0],line[1],4000000*(line[0] as i64)+(line[1] as i64));
            break;
        }
    }
    
}