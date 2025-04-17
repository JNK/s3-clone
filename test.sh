#!/bin/bash
set -e

# Configuration
ENDPOINT="http://localhost:8000"
BUCKET="test-bucket-$(date +%s)"
KEY="hello.txt"
REGION="us-east-1"

# Create a test file
echo "Hello, S3 Clone!" > $KEY

echo "Creating bucket: $BUCKET"
aws --endpoint-url $ENDPOINT --region $REGION s3api create-bucket --bucket $BUCKET

echo "Listing buckets"
aws --endpoint-url $ENDPOINT --region $REGION s3api list-buckets

echo "Uploading file to bucket"
aws --endpoint-url $ENDPOINT --region $REGION s3 cp $KEY s3://$BUCKET/$KEY

echo "Listing objects in bucket"
aws --endpoint-url $ENDPOINT --region $REGION s3 ls s3://$BUCKET

echo "Downloading file from bucket"
aws --endpoint-url $ENDPOINT --region $REGION s3 cp s3://$BUCKET/$KEY downloaded-$KEY

echo "Comparing files"
diff $KEY downloaded-$KEY && echo "Files match!"

# echo "Deleting file from bucket"
# aws --endpoint-url $ENDPOINT --region $REGION s3 rm s3://$BUCKET/$KEY

# echo "Deleting bucket"
# aws --endpoint-url $ENDPOINT --region $REGION s3api delete-bucket --bucket $BUCKET

echo "Cleaning up local files"
rm -f $KEY downloaded-$KEY

echo "Cleaning up test bucket folder in storage"
rm -rf storage/$BUCKET

echo "Test completed successfully."