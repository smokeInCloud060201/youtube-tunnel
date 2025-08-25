package org.example.youtubetunnel.dto;

import lombok.Builder;
import lombok.Data;

@Data
@Builder
public class BaseError {

	private String message;

	private int code;

}
