# Sample Functions Framework C++

## Run Locally with CMake and [vcpkg]:
* install vcpkg
```
git clone https://github.com/Microsoft/vcpkg.git --depth 1
rm -rf ./vcpkg/.git
./vcpkg/bootstrap-vcpkg.sh
export VCPKG_ROOT=$(pwd)/vcpkg
```
* build binary and run
```
cmake -S . -B .build -DCMAKE_TOOLCHAIN_FILE=${VCPKG_ROOT}/scripts/buildsystems/vcpkg.cmake
cmake --build .build
.build/hello
```

## Run Locally with Docker:
* build image and run
```
docker build . kaku-function
docker run -p8080:8080 kaku-function
```

## Deploy
init project config
```
gcloud auth login
gcloud config set project icfpc-gon-the-fox-2024
gcloud config set run/region asia-northeast1
```

```
gcloud run deploy kaku-function --source .
```

```
curl https://kaku-function-pcv2xoziqa-an.a.run.app -H "Authorization: Bearer $(gcloud auth print-identity-token)"
```

Check it out: [http://localhost:8080](http://localhost:8080)

Run on Cloud Run:

[![Run on Google Cloud](https://deploy.cloud.run/button.svg)](https://deploy.cloud.run)
