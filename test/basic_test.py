import socket

# Server address and port
HOST = '127.0.0.1'
PORT = 8080  

def send_command(command):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((HOST, PORT))
        s.sendall(command.encode())  # Send the command to the server
        response = s.recv(1024).decode()  # Receive the response
        return response

def main():
    # Example commands
    insert_command = "INSERT my_key boottyi3323"
    get_command = "LOOKUP my_key"

    # Send INSERT command
    print("Sending:", insert_command)
    insert_response = send_command(insert_command)
    print("Response:", insert_response)

    # Send GET command
    print("Sending:", get_command)
    get_response = send_command(get_command)
    print("Response:", get_response)

if __name__ == "__main__":
    main()
