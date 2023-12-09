
use super::utils::split_input_into_lines;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct DataReading {
    data: Vec<i64>,
}

impl From<String> for DataReading {
    fn from(line: String) -> Self {
        Self {
            data: line.split_ascii_whitespace().map(|n| n.parse().unwrap()).collect()
        }
    }
}

impl DataReading {
    pub fn next_value(self) -> i64 {
        // Add layers until one is filled with 0
        let mut layers = vec![self.data];
        while layers.last().unwrap().iter().any(|value| *value != 0) {
            let mut new_layer = vec![];

            let previous_layer = layers.last().unwrap();
            for i in 0..(previous_layer.len()-1) {
                new_layer.push(previous_layer[i+1] - previous_layer[i]);
            }

            layers.push(new_layer);
        }

        // Compute value
        let mut new_value = 0;
        for layer in layers.into_iter().rev() {
            let last_value = layer.last().unwrap();
            new_value += last_value;
        }
        new_value
    }
}

#[derive(Debug)]
struct SensorReadings {
    readings: Vec<DataReading>,
}

impl From<Vec<String>> for SensorReadings {
    fn from(lines: Vec<String>) -> Self {
        Self {
            readings: lines.into_iter().map(DataReading::from).collect()
        }
    }
}

impl SensorReadings {
    pub fn next_values(self) -> Vec<i64> {
        self.readings.into_iter().map(|r| r.next_value()).collect()
    }
}

pub fn run() -> i64 {
    let readings = SensorReadings::from(split_input_into_lines(INPUT));
    readings.next_values().into_iter().sum()
}
