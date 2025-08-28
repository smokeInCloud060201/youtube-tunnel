package org.example.youtubetunnel.config;

import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.security.config.annotation.web.builders.HttpSecurity;
import org.springframework.security.oauth2.jwt.JwtDecoder;
import org.springframework.security.oauth2.jwt.NimbusJwtDecoder;
import org.springframework.security.web.SecurityFilterChain;
import org.springframework.security.web.authentication.UsernamePasswordAuthenticationFilter;

@Configuration
public class SecurityConfig {

	@Bean
	public SecurityFilterChain securityFilterChain(HttpSecurity http) throws Exception {
		http.authorizeHttpRequests(
				auth -> auth.requestMatchers("/api/public/**").permitAll().anyRequest().authenticated())
			.oauth2ResourceServer(oauth2 -> oauth2.jwt(jwtConfigurer -> jwtConfigurer.decoder(jwtDecoder())));

		return http.build();
	}

	@Bean
	public JwtDecoder jwtDecoder() {
		return NimbusJwtDecoder.withIssuerLocation("https://dev-fcog2dztfmr14aao.us.auth0.com/").build();
	}

}