package org.example.youtubetunnel.search.services.impl;

import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.example.youtubetunnel.config.ConfigProperties;
import org.example.youtubetunnel.search.dto.SearchVideoResponseDTO;
import org.example.youtubetunnel.search.dto.VideoDTO;
import org.example.youtubetunnel.search.services.SearchService;
import org.example.youtubetunnel.utils.WebClientUtil;
import org.springframework.stereotype.Service;

import java.util.Base64;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.CompletableFuture;

@Service
@RequiredArgsConstructor
@Slf4j
public class SearchServiceImpl implements SearchService {

	private final ConfigProperties configProperties;

	private final WebClientUtil webClientUtil;

	private static final String VIDEO_URL = "/youtube/v3/search";

	@Override
	public List<VideoDTO> searchVideo(String part, String type, String query, int maxResults) {

		Map<String, String> queryParams = new HashMap<>();
		queryParams.put("part", part);
		queryParams.put("type", type);
		queryParams.put("query", query);
		queryParams.put("maxResults", String.valueOf(maxResults));
		queryParams.put("key", configProperties.key());

		// SearchVideoResponseDTO searchVideoResponseDTO = webClientUtil.get(VIDEO_URL,
		// queryParams, Collections.emptyMap(), SearchVideoResponseDTO.class).join();
		SearchVideoResponseDTO searchVideoResponseDTO = buildResponse();

		return searchVideoResponseDTO.getItems().stream().map(this::mapItemToVideoDTO).toList();
	}

	private VideoDTO mapItemToVideoDTO(SearchVideoResponseDTO.Item item) {
		if (item == null) {
			return null;
		}

		SearchVideoResponseDTO.Thumbnails thumbnails = item.getSnippet().getThumbnails();

		CompletableFuture<String> defaultThumb = webClientUtil
			.getFullUrl(thumbnails.getDefaultThumbnail().getUrl(), byte[].class)
			.thenApply(bytes -> Base64.getEncoder().encodeToString(bytes));

		CompletableFuture<String> mediumThumb = webClientUtil.getFullUrl(thumbnails.getMedium().getUrl(), byte[].class)
			.thenApply(bytes -> Base64.getEncoder().encodeToString(bytes));

		CompletableFuture<String> highThumb = webClientUtil.getFullUrl(thumbnails.getHigh().getUrl(), byte[].class)
			.thenApply(bytes -> Base64.getEncoder().encodeToString(bytes));

		List<String> base64Thumbnails = CompletableFuture.allOf(defaultThumb, mediumThumb, highThumb)
			.thenApply(v -> List.of(defaultThumb.join(), mediumThumb.join(), highThumb.join()))
			.join();

		return VideoDTO.builder()
			.publishTime(item.getSnippet().getPublishTime())
			.publishedAt(item.getSnippet().getPublishedAt())
			.title(item.getSnippet().getTitle())
			.id(item.getId().getVideoId())
			.description(item.getSnippet().getDescription())
			.channelTitle(item.getSnippet().getChannelTitle())
			.thumbnails(base64Thumbnails)
			.build();
	}

	public SearchVideoResponseDTO buildResponse() {
		SearchVideoResponseDTO.Thumbnails.Thumbnail defaultThumb1 = new SearchVideoResponseDTO.Thumbnails.Thumbnail();
		defaultThumb1.setUrl("https://i.ytimg.com/vi/NJuNJ8Xs_00/default.jpg");
		defaultThumb1.setWidth(120);
		defaultThumb1.setHeight(90);

		SearchVideoResponseDTO.Thumbnails.Thumbnail mediumThumb1 = new SearchVideoResponseDTO.Thumbnails.Thumbnail();
		mediumThumb1.setUrl("https://i.ytimg.com/vi/NJuNJ8Xs_00/mqdefault.jpg");
		mediumThumb1.setWidth(320);
		mediumThumb1.setHeight(180);

		SearchVideoResponseDTO.Thumbnails.Thumbnail highThumb1 = new SearchVideoResponseDTO.Thumbnails.Thumbnail();
		highThumb1.setUrl("https://i.ytimg.com/vi/NJuNJ8Xs_00/hqdefault.jpg");
		highThumb1.setWidth(480);
		highThumb1.setHeight(360);

		SearchVideoResponseDTO.Thumbnails thumbnails1 = new SearchVideoResponseDTO.Thumbnails();
		thumbnails1.setDefaultThumbnail(defaultThumb1);
		thumbnails1.setMedium(mediumThumb1);
		thumbnails1.setHigh(highThumb1);

		SearchVideoResponseDTO.Snippet snippet1 = new SearchVideoResponseDTO.Snippet();
		snippet1.setPublishedAt("2025-08-19T13:11:40Z");
		snippet1.setChannelId("UCzEhWhrRUdRgxh_UuFpUuzQ");
		snippet1.setTitle("NHÀ TÔI CÓ TREO MỘT LÁ CỜ - DTAP x HÀ ANH TUẤN | OFFICIAL MV");
		snippet1.setDescription(
				"Một trong những khúc ca xúc động nhất album. Tình yêu quốc kỳ được thắp sáng – lá cờ theo chân người Việt ra khắp năm châu, ...");
		snippet1.setThumbnails(thumbnails1);
		snippet1.setChannelTitle("DTAP OFFICIAL");
		snippet1.setLiveBroadcastContent("none");
		snippet1.setPublishTime("2025-08-19T13:11:40Z");

		SearchVideoResponseDTO.Id id1 = new SearchVideoResponseDTO.Id();
		id1.setKind("youtube#video");
		id1.setVideoId("NJuNJ8Xs_00");

		SearchVideoResponseDTO.Item item1 = new SearchVideoResponseDTO.Item();
		item1.setKind("youtube#searchResult");
		item1.setEtag("gsinbJ-L-asB49XUevXFEzQI9oo");
		item1.setId(id1);
		item1.setSnippet(snippet1);

		// -------------------- Video 2 --------------------
		SearchVideoResponseDTO.Thumbnails.Thumbnail defaultThumb2 = new SearchVideoResponseDTO.Thumbnails.Thumbnail();
		defaultThumb2.setUrl("https://i.ytimg.com/vi/r2gifXGsIbg/default.jpg");
		defaultThumb2.setWidth(120);
		defaultThumb2.setHeight(90);

		SearchVideoResponseDTO.Thumbnails.Thumbnail mediumThumb2 = new SearchVideoResponseDTO.Thumbnails.Thumbnail();
		mediumThumb2.setUrl("https://i.ytimg.com/vi/r2gifXGsIbg/mqdefault.jpg");
		mediumThumb2.setWidth(320);
		mediumThumb2.setHeight(180);

		SearchVideoResponseDTO.Thumbnails.Thumbnail highThumb2 = new SearchVideoResponseDTO.Thumbnails.Thumbnail();
		highThumb2.setUrl("https://i.ytimg.com/vi/r2gifXGsIbg/hqdefault.jpg");
		highThumb2.setWidth(480);
		highThumb2.setHeight(360);

		SearchVideoResponseDTO.Thumbnails thumbnails2 = new SearchVideoResponseDTO.Thumbnails();
		thumbnails2.setDefaultThumbnail(defaultThumb2);
		thumbnails2.setMedium(mediumThumb2);
		thumbnails2.setHigh(highThumb2);

		SearchVideoResponseDTO.Snippet snippet2 = new SearchVideoResponseDTO.Snippet();
		snippet2.setPublishedAt("2025-08-22T23:37:38Z");
		snippet2.setChannelId("UCCZZuYT8pZED_3bNasz7RdA");
		snippet2.setTitle("Nhà tôi có treo một lá cờ - Hà Anh Tuấn | Cầu truyền hình Thời cơ Vàng");
		snippet2.setDescription(
				"vtvshows #haanhtuan Nhà tôi có một lá cờ - Hà Anh Tuấn | Cầu truyền hình Thời cơ Vàng #VTVSHOWS là kênh giải trí số của Đài ...");
		snippet2.setThumbnails(thumbnails2);
		snippet2.setChannelTitle("VTV SHOWS");
		snippet2.setLiveBroadcastContent("none");
		snippet2.setPublishTime("2025-08-22T23:37:38Z");

		SearchVideoResponseDTO.Id id2 = new SearchVideoResponseDTO.Id();
		id2.setKind("youtube#video");
		id2.setVideoId("r2gifXGsIbg");

		SearchVideoResponseDTO.Item item2 = new SearchVideoResponseDTO.Item();
		item2.setKind("youtube#searchResult");
		item2.setEtag("vIgbMjsIH8BiS3pLMpVbMkvuXxk");
		item2.setId(id2);
		item2.setSnippet(snippet2);

		// -------------------- Video 3 --------------------
		SearchVideoResponseDTO.Thumbnails.Thumbnail defaultThumb3 = new SearchVideoResponseDTO.Thumbnails.Thumbnail();
		defaultThumb3.setUrl("https://i.ytimg.com/vi/8D83WpIbixc/default.jpg");
		defaultThumb3.setWidth(120);
		defaultThumb3.setHeight(90);

		SearchVideoResponseDTO.Thumbnails.Thumbnail mediumThumb3 = new SearchVideoResponseDTO.Thumbnails.Thumbnail();
		mediumThumb3.setUrl("https://i.ytimg.com/vi/8D83WpIbixc/mqdefault.jpg");
		mediumThumb3.setWidth(320);
		mediumThumb3.setHeight(180);

		SearchVideoResponseDTO.Thumbnails.Thumbnail highThumb3 = new SearchVideoResponseDTO.Thumbnails.Thumbnail();
		highThumb3.setUrl("https://i.ytimg.com/vi/8D83WpIbixc/hqdefault.jpg");
		highThumb3.setWidth(480);
		highThumb3.setHeight(360);

		SearchVideoResponseDTO.Thumbnails thumbnails3 = new SearchVideoResponseDTO.Thumbnails();
		thumbnails3.setDefaultThumbnail(defaultThumb3);
		thumbnails3.setMedium(mediumThumb3);
		thumbnails3.setHigh(highThumb3);

		SearchVideoResponseDTO.Snippet snippet3 = new SearchVideoResponseDTO.Snippet();
		snippet3.setPublishedAt("2025-08-23T13:00:23Z");
		snippet3.setChannelId("UCKxnLEsKRcuybpdbWv3dnww");
		snippet3.setTitle("NHÀ TÔI CÓ TREO MỘT LÁ CỜ | Ca sĩ Hà Anh Tuấn | Cầu truyền hình THỜI CƠ VÀNG");
		snippet3.setDescription(
				"NHÀ TÔI CÓ TREO MỘT LÁ CỜ | Ca sĩ Hà Anh Tuấn | Cầu truyền hình THỜI CƠ VÀNG NHÀ TÔI CÓ TREO MỘT LÁ CỜ Sáng tác ...");
		snippet3.setThumbnails(thumbnails3);
		snippet3.setChannelTitle("HOA XUÂN CA");
		snippet3.setLiveBroadcastContent("none");
		snippet3.setPublishTime("2025-08-23T13:00:23Z");

		SearchVideoResponseDTO.Id id3 = new SearchVideoResponseDTO.Id();
		id3.setKind("youtube#video");
		id3.setVideoId("8D83WpIbixc");

		SearchVideoResponseDTO.Item item3 = new SearchVideoResponseDTO.Item();
		item3.setKind("youtube#searchResult");
		item3.setEtag("UnUFo-QBKW6nFXqD5Dz18KoIUl4");
		item3.setId(id3);
		item3.setSnippet(snippet3);

		// -------------------- Video 4 --------------------
		SearchVideoResponseDTO.Thumbnails.Thumbnail defaultThumb4 = new SearchVideoResponseDTO.Thumbnails.Thumbnail();
		defaultThumb4.setUrl("https://i.ytimg.com/vi/nOzI4TyWEGw/default.jpg");
		defaultThumb4.setWidth(120);
		defaultThumb4.setHeight(90);

		SearchVideoResponseDTO.Thumbnails.Thumbnail mediumThumb4 = new SearchVideoResponseDTO.Thumbnails.Thumbnail();
		mediumThumb4.setUrl("https://i.ytimg.com/vi/nOzI4TyWEGw/mqdefault.jpg");
		mediumThumb4.setWidth(320);
		mediumThumb4.setHeight(180);

		SearchVideoResponseDTO.Thumbnails.Thumbnail highThumb4 = new SearchVideoResponseDTO.Thumbnails.Thumbnail();
		highThumb4.setUrl("https://i.ytimg.com/vi/nOzI4TyWEGw/hqdefault.jpg");
		highThumb4.setWidth(480);
		highThumb4.setHeight(360);

		SearchVideoResponseDTO.Thumbnails thumbnails4 = new SearchVideoResponseDTO.Thumbnails();
		thumbnails4.setDefaultThumbnail(defaultThumb4);
		thumbnails4.setMedium(mediumThumb4);
		thumbnails4.setHigh(highThumb4);

		SearchVideoResponseDTO.Snippet snippet4 = new SearchVideoResponseDTO.Snippet();
		snippet4.setPublishedAt("2025-08-24T01:00:56Z");
		snippet4.setChannelId("UCl-Wf44-szOK5QjSmQ2KCug");
		snippet4
			.setTitle("HÀ ANH TUẤN NGÂN VANG CA KHÚC “NHÀ TÔI CÓ TREO MỘT LÁ CỜ” TRONG CHƯƠNG TRÌNH \"THỜI CƠ VÀNG\"");
		snippet4.setDescription(
				"Nhà tôi có treo một lá cờ” – ca khúc đầy tự hào được ca sĩ Hà Anh Tuấn thể hiện trong chương trình Thời Cơ Vàng, giữa không ...");
		snippet4.setThumbnails(thumbnails4);
		snippet4.setChannelTitle("VTV3");
		snippet4.setLiveBroadcastContent("none");
		snippet4.setPublishTime("2025-08-24T01:00:56Z");

		SearchVideoResponseDTO.Id id4 = new SearchVideoResponseDTO.Id();
		id4.setKind("youtube#video");
		id4.setVideoId("nOzI4TyWEGw");

		SearchVideoResponseDTO.Item item4 = new SearchVideoResponseDTO.Item();
		item4.setKind("youtube#searchResult");
		item4.setEtag("zfBGpc7e9plpJNn-Kqw1FQIlccU");
		item4.setId(id4);
		item4.setSnippet(snippet4);

		// -------------------- Video 5 --------------------
		SearchVideoResponseDTO.Thumbnails.Thumbnail defaultThumb5 = new SearchVideoResponseDTO.Thumbnails.Thumbnail();
		defaultThumb5.setUrl("https://i.ytimg.com/vi/PnwR68I-s64/default.jpg");
		defaultThumb5.setWidth(120);
		defaultThumb5.setHeight(90);

		SearchVideoResponseDTO.Thumbnails.Thumbnail mediumThumb5 = new SearchVideoResponseDTO.Thumbnails.Thumbnail();
		mediumThumb5.setUrl("https://i.ytimg.com/vi/PnwR68I-s64/mqdefault.jpg");
		mediumThumb5.setWidth(320);
		mediumThumb5.setHeight(180);

		SearchVideoResponseDTO.Thumbnails.Thumbnail highThumb5 = new SearchVideoResponseDTO.Thumbnails.Thumbnail();
		highThumb5.setUrl("https://i.ytimg.com/vi/PnwR68I-s64/hqdefault.jpg");
		highThumb5.setWidth(480);
		highThumb5.setHeight(360);

		SearchVideoResponseDTO.Thumbnails thumbnails5 = new SearchVideoResponseDTO.Thumbnails();
		thumbnails5.setDefaultThumbnail(defaultThumb5);
		thumbnails5.setMedium(mediumThumb5);
		thumbnails5.setHigh(highThumb5);

		SearchVideoResponseDTO.Snippet snippet5 = new SearchVideoResponseDTO.Snippet();
		snippet5.setPublishedAt("2025-08-23T00:00:25Z");
		snippet5.setChannelId("UCUjatsUCk1OchBd9YXvQOnQ");
		snippet5.setTitle("Nhà tôi có treo một lá cờ | Hà Anh Tuấn – DTAP | Cầu Truyền hình trực tiếp Thời cơ vàng");
		snippet5.setDescription(
				"rangrovietnam #VTVdigital Nhà tôi có treo một lá cờ | Hà Anh Tuấn – DTAP | Cầu Truyền hình trực tiếp Thời cơ vàng \"Rạng rỡ ...");
		snippet5.setThumbnails(thumbnails5);
		snippet5.setChannelTitle("Rạng rỡ Việt Nam");
		snippet5.setLiveBroadcastContent("none");
		snippet5.setPublishTime("2025-08-23T00:00:25Z");

		SearchVideoResponseDTO.Id id5 = new SearchVideoResponseDTO.Id();
		id5.setKind("youtube#video");
		id5.setVideoId("PnwR68I-s64");

		SearchVideoResponseDTO.Item item5 = new SearchVideoResponseDTO.Item();
		item5.setKind("youtube#searchResult");
		item5.setEtag("TZdCz_LUoBcS57kcnQVjl9rh5eg");
		item5.setId(id5);
		item5.setSnippet(snippet5);

		// -------------------- PageInfo --------------------
		SearchVideoResponseDTO.PageInfo pageInfo = new SearchVideoResponseDTO.PageInfo();
		pageInfo.setTotalResults(1000000);
		pageInfo.setResultsPerPage(5);

		// -------------------- Main Response --------------------
		SearchVideoResponseDTO response = new SearchVideoResponseDTO();
		response.setKind("youtube#searchListResponse");
		response.setEtag("FHBeVBzxinxss22DVIGPs4cuWVs");
		response.setNextPageToken("CAUQAA");
		response.setRegionCode("VN");
		response.setPageInfo(pageInfo);
		response.setItems(List.of(item1, item2, item3, item4, item5));

		return response;
	}

}
