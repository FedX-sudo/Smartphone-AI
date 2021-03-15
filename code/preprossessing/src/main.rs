use std::fs::File;
use std::io::Read;
use std::error::Error;
use csv::WriterBuilder;

fn example() -> Result<(), Box<Error>> {
    let mut wtr = Writer::from_path("foo.csv")?;
    wtr.write_record(&["a", "b", "c"])?;
    wtr.write_record(&["x", "y", "z"])?;
    wtr.flush()?;
    Ok(())
}
fn main() {
    let mut i = 0; 
    
    //TODO finish this:
    let files = ["/home/fenixg/Code/cyberReachLabs/smartphoneTHing-cyberReaach1/Motion sensor data preprocessing - Module/CSM_MotionSensor_Dataset/CSM_MotionSensor_Dataset/User1Motion.txt", "/home/fenixg/Code/cyberReachLabs/smartphoneTHing-cyberReaach1/Motion sensor data preprocessing - Module/CSM_MotionSensor_Dataset/CSM_MotionSensor_Dataset/User2Motion.txt", "/home/fenixg/Code/cyberReachLabs/smartphoneTHing-cyberReaach1/Motion sensor data preprocessing - Module/CSM_MotionSensor_Dataset/CSM_MotionSensor_Dataset/User3Motion.txt", "/home/fenixg/Code/cyberReachLabs/smartphoneTHing-cyberReaach1/Motion sensor data preprocessing - Module/CSM_MotionSensor_Dataset/CSM_MotionSensor_Dataset/User4Motion.txt", "/home/fenixg/Code/cyberReachLabs/smartphoneTHing-cyberReaach1/Motion sensor data preprocessing - Module/CSM_MotionSensor_Dataset/CSM_MotionSensor_Dataset/User5Motion.txt", "/home/fenixg/Code/cyberReachLabs/smartphoneTHing-cyberReaach1/Motion sensor data preprocessing - Module/CSM_MotionSensor_Dataset/CSM_MotionSensor_Dataset/User6Motion.txt", "/home/fenixg/Code/cyberReachLabs/smartphoneTHing-cyberReaach1/Motion sensor data preprocessing - Module/CSM_MotionSensor_Dataset/CSM_MotionSensor_Dataset/User7Motion.txt", "/home/fenixg/Code/cyberReachLabs/smartphoneTHing-cyberReaach1/Motion sensor data preprocessing - Module/CSM_MotionSensor_Dataset/CSM_MotionSensor_Dataset/User8Motion.txt", "/home/fenixg/Code/cyberReachLabs/smartphoneTHing-cyberReaach1/Motion sensor data preprocessing - Module/CSM_MotionSensor_Dataset/CSM_MotionSensor_Dataset/User9Motion.txt", "/home/fenixg/Code/cyberReachLabs/smartphoneTHing-cyberReaach1/Motion sensor data preprocessing - Module/CSM_MotionSensor_Dataset/CSM_MotionSensor_Dataset/User10Motion.txt", "/home/fenixg/Code/cyberReachLabs/smartphoneTHing-cyberReaach1/Motion sensor data preprocessing - Module/CSM_MotionSensor_Dataset/CSM_MotionSensor_Dataset/User11Motion.txt", "/home/fenixg/Code/cyberReachLabs/smartphoneTHing-cyberReaach1/Motion sensor data preprocessing - Module/CSM_MotionSensor_Dataset/CSM_MotionSensor_Dataset/User12Motion.txt", "/home/fenixg/Code/cyberReachLabs/smartphoneTHing-cyberReaach1/Motion sensor data preprocessing - Module/CSM_MotionSensor_Dataset/CSM_MotionSensor_Dataset/User13Motion.txt", "/home/fenixg/Code/cyberReachLabs/smartphoneTHing-cyberReaach1/Motion sensor data preprocessing - Module/CSM_MotionSensor_Dataset/CSM_MotionSensor_Dataset/User14Motion.txt", "/home/fenixg/Code/cyberReachLabs/smartphoneTHing-cyberReaach1/Motion sensor data preprocessing - Module/CSM_MotionSensor_Dataset/CSM_MotionSensor_Dataset/User15Motion.txt", "/home/fenixg/Code/cyberReachLabs/smartphoneTHing-cyberReaach1/Motion sensor data preprocessing - Module/CSM_MotionSensor_Dataset/CSM_MotionSensor_Dataset/User16Motion.txt", "/home/fenixg/Code/cyberReachLabs/smartphoneTHing-cyberReaach1/Motion sensor data preprocessing - Module/CSM_MotionSensor_Dataset/CSM_MotionSensor_Dataset/User17Motion.txt", "/home/fenixg/Code/cyberReachLabs/smartphoneTHing-cyberReaach1/Motion sensor data preprocessing - Module/CSM_MotionSensor_Dataset/CSM_MotionSensor_Dataset/User18Motion.txt", "/home/fenixg/Code/cyberReachLabs/smartphoneTHing-cyberReaach1/Motion sensor data preprocessing - Module/CSM_MotionSensor_Dataset/CSM_MotionSensor_Dataset/User19Motion.txt", "/home/fenixg/Code/cyberReachLabs/smartphoneTHing-cyberReaach1/Motion sensor data preprocessing - Module/CSM_MotionSensor_Dataset/CSM_MotionSensor_Dataset/User20Motion.txt", "/home/fenixg/Code/cyberReachLabs/smartphoneTHing-cyberReaach1/Motion sensor data preprocessing - Module/CSM_MotionSensor_Dataset/CSM_MotionSensor_Dataset/User21Motion.txt", "/home/fenixg/Code/cyberReachLabs/smartphoneTHing-cyberReaach1/Motion sensor data preprocessing - Module/CSM_MotionSensor_Dataset/CSM_MotionSensor_Dataset/User22Motion.txt", "/home/fenixg/Code/cyberReachLabs/smartphoneTHing-cyberReaach1/Motion sensor data preprocessing - Module/CSM_MotionSensor_Dataset/CSM_MotionSensor_Dataset/User23Motion.txt"];
 
    let mut i = 0;
    loop{
        let filename = files[i];
        // Open the file in read-only mode.
        match File::open(filename) {
            // The file is open (no error).
            Ok(mut file) => {
                let mut content = String::new();

                // Read all the file content into a variable (ignoring the result of the operation).
                file.read_to_string(&mut content).unwrap();

                println!("{}", content);

                // The file is automatically closed when is goes out of scope.
            },
            // Error handling.
            Err(error) => {
                println!("Error opening file {}: {}", filename, error);
            },
        }
    i+=1;
    if i == 1{break}
    
    }
}

