package com.dev.youtubetunnel;

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.boot.context.properties.ConfigurationPropertiesScan;

@ConfigurationPropertiesScan
@SpringBootApplication
public class MainserviceApplication {

    public static void main(String[] args) {
        SpringApplication.run(MainserviceApplication.class, args);
    }

}
