package com.github.silentsokolov;

import java.io.IOException;
import org.apache.spark.sql.api.java.UDF1;


public class App implements UDF1<String, String[]> {
    public static native String[] nativeUDF(String url);

    static {
        try {
            if (OperatingSystem.isMac()) {
                NativeUtils.loadLibraryFromJar("/libs/darwin-amd64.dylib");
            } else if (OperatingSystem.isUnix()) {
                NativeUtils.loadLibraryFromJar("/libs/linux-amd64.so");
            };
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    @Override
    public String[] call(String url) throws Exception {
        return App.nativeUDF(url);
    }
}