import time
import threading
import http.server
import socketserver
import os

# Configuration
PORT = 8000
HTML_FILE = "/home/root/index.html"
DEVICE_FILE = "/sys/bus/w1/devices/28-000001cda180/w1_slave"
UPDATE_INTERVAL = 1  # Update HTML every 1 second

def read_temp():
    try:
        with open(DEVICE_FILE, 'r') as f:
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

def generate_html(temp):
    formatted_temp = "N/A" if temp is None else f"{temp:.3f} °C"
    html_content = f"""
    <!DOCTYPE html>
    <html>
    <head>
        <meta http-equiv="refresh" content="1">
    </head>
    <body>
    {formatted_temp}
    </body>
    </html>
    """
    with open(HTML_FILE, 'w') as f:
        f.write(html_content)

def update_html():
    while True:
        temp = read_temp()
        generate_html(temp)
        #print(f"Temperature: {temp:.3f} °C" if temp is not None else "Failed to read temperature")
        time.sleep(UPDATE_INTERVAL)

def start_server():
    os.chdir("/home/root")  # Serve from /home/root
    Handler = http.server.SimpleHTTPRequestHandler
    with socketserver.TCPServer(("", PORT), Handler) as httpd:
        print(f"Serving at http://192.168.0.2:{PORT}")
        httpd.serve_forever()

if __name__ == "__main__":
    # Start HTML update in a separate thread
    update_thread = threading.Thread(target=update_html, daemon=True)
    update_thread.start()
    # Start web server
    start_server()
