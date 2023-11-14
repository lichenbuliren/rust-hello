use serde::Deserialize;
use serde_json::Value;
use std::fs::{self, File};
use std::io::Read;


#[derive(Debug, Deserialize)]
struct PerformanceMetrics {
    homepage_fcp: i32,
    homepage_tti: i32,
    homepage_lcp: i32,
    use_overview: i32,
    use_module_list: i32,
    disk_cache: i32,
    total_cache: i32
}

fn calculate_average_cost(structs: &[PerformanceMetrics]) -> PerformanceMetrics {
    let total_homepage_fcp: i32 = structs.iter().map(|s: &PerformanceMetrics| s.homepage_fcp).sum();
    let total_homepage_tti: i32 = structs.iter().map(|s: &PerformanceMetrics| s.homepage_tti).sum();
    let total_homepage_lcp: i32 = structs.iter().map(|s: &PerformanceMetrics| s.homepage_lcp).sum();
    let total_use_overview: i32 = structs.iter().map(|s: &PerformanceMetrics| s.use_overview).sum();
    let total_use_module_list: i32 = structs.iter().map(|s: &PerformanceMetrics| s.use_module_list).sum();
    let total_disk_cache: i32 = structs.iter().map(|s: &PerformanceMetrics| s.disk_cache).sum();
    let total_total_cache: i32 = structs.iter().map(|s: &PerformanceMetrics| s.total_cache).sum();
    let num_structs = structs.len() as i32;

    if num_structs > 0 {
        let average_fcp = total_homepage_fcp / num_structs;
        let average_tti = total_homepage_tti / num_structs;
        let average_lcp = total_homepage_lcp / num_structs;
        let average_overview = total_use_overview / num_structs;
        let average_module_list = total_use_module_list / num_structs;
        let average_disk_cache = total_disk_cache / num_structs;
        let average_total_cache = total_total_cache / num_structs;
        PerformanceMetrics { 
            homepage_fcp: average_fcp,
            homepage_tti: average_tti,
            homepage_lcp: average_lcp,
            use_overview: average_overview,
            use_module_list: average_module_list,
            disk_cache: average_disk_cache,
            total_cache: average_total_cache
        }
    } else {
        PerformanceMetrics {
            homepage_fcp: 0,
            homepage_tti: 0,
            homepage_lcp: 0,
            use_overview: 0,
            use_module_list: 0,
            disk_cache: 0,
            total_cache: 0
        }
    }
}

fn parse_data(json_str: Value) -> Vec<PerformanceMetrics> {
    let mut metrics_arr: Vec<PerformanceMetrics> = vec![];
    if let Value::Array(arr) = json_str {
        for item in arr {
            if let Some(data) = item.get("data") {
                if let Some(flow) = data.get("flow") {
                    if let Some(flow_item) = flow[0].as_str() {
                        if flow_item == "Homepage LCP report" {
                            if let Some(performance_message) = data.get("message") {
                                let mut metrics = PerformanceMetrics {
                                    homepage_fcp: 0,
                                    homepage_tti: 0,
                                    homepage_lcp: 0,
                                    use_overview: 0,
                                    use_module_list: 0,
                                    disk_cache: 0,
                                    total_cache: 0,
                                };

                                let lines: Vec<&str> = performance_message.as_str().unwrap().trim().split('\n').collect();

                                for line in lines {
                                    let parts: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
                                    if parts.len() == 2 {
                                        let key = parts[0];
                                        let value = parts[1].parse::<i32>().unwrap();
                            
                                        if value > 0 {
                                            match key {
                                                "homepageFCP" => metrics.homepage_fcp = value,
                                                "homepageTTI" => metrics.homepage_tti = value,
                                                "homepageLCP" => metrics.homepage_lcp = value,
                                                "useOverview" => metrics.use_overview = value,
                                                "useModuleList" => metrics.use_module_list = value,
                                                "diskCache" => metrics.disk_cache = value,
                                                "totalCache" => metrics.total_cache = value,
                                                _ => {}
                                            }
                                        }
                                    }
                                }

                                let all_greater_than_zero = [
                                    metrics.homepage_fcp,
                                    metrics.homepage_tti,
                                    metrics.homepage_lcp,
                                    metrics.use_overview,
                                    metrics.use_module_list,
                                    metrics.disk_cache,
                                    metrics.total_cache,
                                ]
                                .iter()
                                .all(|&x| x > 0);

                                if all_greater_than_zero {
                                    metrics_arr.push(metrics);
                                }
                                
                            }
                        }
                    }
                }
            }
        }
    }

    return metrics_arr;
}

fn main() {

    let dir_path = "optimize-before";

    let entries = fs::read_dir(dir_path).expect("无法读取目录");

    let mut final_result = Vec::<PerformanceMetrics>::new();
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "txt") {
                let mut file = File::open(&path).expect("无法打开文件");
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("无法读取文件");

                let json_data: Value = serde_json::from_str(&contents).expect("无法解析 JSON");

                let mut performance_content: Vec<PerformanceMetrics> = parse_data(json_data);
                final_result.append(&mut performance_content);
            }
        }
    }

    // 这里采用 &final_result，不然后续无法继续使用该变量
    for result in &final_result {
        println!("final_result {:?}", result);
    }

    let average_performance = calculate_average_cost(&final_result);

    println!("average_performance {:?}", average_performance);
    
}
