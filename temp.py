import time                                                                                        
                                                                                                   
def read_temp():                                                                                   
    try:                                                                                           
        with open('/sys/bus/w1/devices/28-000001cda180/w1_slave', 'r') as f:                       
            lines = f.readlines()                                                                  
        if lines[0].strip().endswith('YES'):                                                       
            temp_str = lines[1].split('t=')[1].strip()                                             
            temp_c = float(temp_str) / 1000.0                                                      
            return temp_c                                                                          
        else:                                                                                      
            return None                                                                            
    except Exception as e:                                                                         
        print(f"Error reading temperature: {e}")                                                   
        return None                                                                                
                                                                                                   
while True:                                                                                        
    temp = read_temp()                                                                             
    if temp is not None:                                                                           
        print(f"Temperature: {temp:.3f} °C")                                                       
    else:                                                                                          
        print("Failed to read temperature")                                                        
    time.sleep(1)                                                                                  
                                                                                                   
                                                                                                   

