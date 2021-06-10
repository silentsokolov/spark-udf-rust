[![build](https://github.com/silentsokolov/spark-udf-rust/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/silentsokolov/spark-udf-rust/actions/workflows/build.yml)

# Apache Spark UDFs in Rust

This repo is an example (and template) for using the Rust language to write Spark UDFs. 

### WTF?

Ok, I know this is not the best solution and rather Scala or Java should be used. But the JVM world is very big (_for me_). I use Python in my job and sometimes Python UDFs are slow that's why this project was born.

# Usage Notes

### Step 1: Clone repo

`git clone <url>`

### Step 2: Rename rustside and javaside (Optional)

You can rename `rustside` and `javaside` if needed.

### Step 3: Fix App class

You need to update Java UDF (in file `App.java`). Specify the correct input / output types. In the current implementation an input type is `String` and an output type is `[]String`. This is simple, you don't need to know Java.

### Step 4: Update C headers

Run `make java_compile`

It's needed for implementing a function in Rust. You can see a file with headers here `${JAVASIDE}/src/main/native/include/com_github_silentsokolov_App.h`.

### Step 5: Implement Rust

We use the [jni-rs](https://docs.rs/jni/0.19.0/jni/). Read docs and implement a function in Rust.

You can run `make rust_test` and `make rust_build` for test/build.

### Step 6: Build Jar

Run `make build`

### Step 6: Register and use your UDF

```python
# spark-submit --jars /path/to/udf.jar ...

spark.udf.registerJavaFunction('parseUrl', 'com.github.silentsokolov.App', t.ArrayType(t.StringType(), True))

df = spark.createDataFrame(
    [
        ('https://www.github.com/rust-lang/rust/issues?labels=E-easy&state=open#hash'),
        ('https://google.com/'),
    ],
    ['url']
)
df = df.select(f.expr('parseUrl(url)'))
df.show()
```

### Step 6: Bonus

You can also use this repo to automatically create a jar with Github Action. Just create a git tag that starts with `v`.

# Benchmark

```
python: 15.3 s ± 396 ms per loop (mean ± std. dev. of 7 runs, 1 loop each)
native: 8.98 s ± 375 ms per loop (mean ± std. dev. of 7 runs, 1 loop each)
rust: 7.84 s ± 340 ms per loop (mean ± std. dev. of 7 runs, 1 loop each)
```
