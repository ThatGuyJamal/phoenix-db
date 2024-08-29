import socket

def send_command(command: str):
    # Define the server address and port
    server_address = ('127.0.0.1', 7878)
    
    # Create a TCP/IP socket
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
        # Connect the socket to the server address and port
        sock.connect(server_address)
        
        # Send the command to the server
        sock.sendall(command.encode('utf-8') + b'\n')
        
        # Receive the response from the server
        response = sock.recv(1024).decode('utf-8')
        print(f"Response for '{command}': {response.strip()}")

# Test commands
commands = [
    "SET mykey myvalue",
    "GET mykey",
    "DEL mykey",
    "GET mykey",
    "LIST",
    "HELP",
    "EXIT"
]

# Send each command and print the response
for cmd in commands:
    send_command(cmd)
