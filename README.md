# S3-Compatible Server

A Rust implementation of an S3-compatible server that uses the local filesystem for storage. This server implements the following S3 operations:

- ListBuckets
- ListObjects
- ListObjectsV2
- GetObject
- PutObject
- UploadPart

## Features

- Local filesystem storage
- AWS signature v4 authentication
- YAML-based configuration
- No database required
- Real-time filesystem synchronization

## Prerequisites

- Rust and Cargo (latest stable version)

## Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/s3-clone.git
   cd s3-clone
   ```

2. Create a configuration file `config.yaml`:
   ```yaml
   server:
     host: "127.0.0.1"
     port: 8000
     storage_path: "./storage"

   credentials:
     - access_key_id: "test-key"
       secret_access_key: "test-secret"
       permissions:
         - action: "*"
           resource: "*"
   ```

3. Build and run the server:
   ```bash
   cargo build --release
   cargo run --release
   ```

## Configuration

The server is configured through `config.yaml`. You can add multiple credentials with different permissions:

```yaml
credentials:
  - access_key_id: "readonly-key"
    secret_access_key: "readonly-secret"
    permissions:
      - action: "GetObject"
        resource: "*"
      - action: "ListBuckets"
        resource: "*"
      - action: "ListObjects"
        resource: "*"
```

Available actions:
- `ListBuckets`
- `ListObjects`
- `ListObjectsV2`
- `GetObject`
- `PutObject`
- `UploadPart`

## Using with AWS CLI

You can use the AWS CLI to interact with the server:

1. Configure AWS CLI with your credentials:
   ```bash
   aws configure set aws_access_key_id test-key
   aws configure set aws_secret_access_key test-secret
   aws configure set region us-east-1
   ```

2. Create a bucket:
   ```bash
   aws s3 mb s3://my-bucket --endpoint-url http://localhost:8000
   ```

3. Upload a file:
   ```bash
   aws s3 cp myfile.txt s3://my-bucket/ --endpoint-url http://localhost:8000
   ```

4. List objects in a bucket:
   ```bash
   aws s3 ls s3://my-bucket --endpoint-url http://localhost:8000
   ```

5. Download a file:
   ```bash
   aws s3 cp s3://my-bucket/myfile.txt downloaded.txt --endpoint-url http://localhost:8000
   ```

## Using with AWS SDK

You can use any AWS SDK to interact with this server. Here's an example using the AWS SDK for Python (boto3):

```python
import boto3

s3 = boto3.client('s3',
    aws_access_key_id='test-key',
    aws_secret_access_key='test-secret',
    endpoint_url='http://localhost:8000'
)

# List buckets
response = s3.list_buckets()
for bucket in response['Buckets']:
    print(bucket['Name'])

# Upload a file
s3.upload_file('myfile.txt', 'my-bucket', 'myfile.txt')

# Download a file
s3.download_file('my-bucket', 'myfile.txt', 'downloaded.txt')
```

## Storage Structure

The server uses a simple filesystem structure:

```
storage/
  bucket1/
    file1.txt
    file2.jpg
    dir/
      file3.png
  bucket2/
    file4.txt
```

Each bucket is a directory under the storage path, and objects are stored with their full key path preserved.

## Limitations

- Multipart uploads are partially implemented (only UploadPart operation)
- No support for bucket policies
- No support for object versioning
- No support for object tagging
- Basic authentication implementation
