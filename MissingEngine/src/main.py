import sys
import dotenv

import services.file_descriptor as file_descriptor

dotenv.load_dotenv()

if len(sys.argv) < 2:
    print("Usage: python main.py <path to file>")
    sys.exit(1)

filepath = sys.argv[1]
content = file_descriptor.read_file(filepath)
jsonContent = file_descriptor.json_decrypt(content)

file_descriptor.write_file(filepath + "_result", content)
print(filepath + '_result')