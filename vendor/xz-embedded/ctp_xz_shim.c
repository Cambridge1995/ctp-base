/*
 * ctp-base 构建脚本使用的 xz 解压入口。
 *
 * 基于 xz-embedded（公有领域）实现文件到文件的流式解压，
 * 避免为构建脚本引入第三方 Rust crate。
 */

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include "xz.h"

int ctp_xz_decompress_file(const char *src_path, const char *dst_path)
{
	FILE *in;
	FILE *out;
	struct xz_dec *dec;
	struct xz_buf buf;
	uint8_t in_buf[8192];
	uint8_t out_buf[16384];
	enum xz_ret ret;
	int result = 0;

	in = fopen(src_path, "rb");
	if (in == NULL)
		return 1;
	out = fopen(dst_path, "wb");
	if (out == NULL) {
		fclose(in);
		return 2;
	}
	dec = xz_dec_init(XZ_DYNALLOC, 32U << 20);
	if (dec == NULL) {
		fclose(in);
		fclose(out);
		return 3;
	}

	buf.in = in_buf;
	buf.in_pos = 0;
	buf.in_size = 0;
	buf.out = out_buf;
	buf.out_pos = 0;
	buf.out_size = sizeof(out_buf);

	for (;;) {
		if (buf.in_pos == buf.in_size) {
			buf.in_size = fread(in_buf, 1, sizeof(in_buf), in);
			buf.in_pos = 0;
			if (buf.in_size == 0) {
				result = 4;
				break;
			}
		}
		ret = xz_dec_run(dec, &buf);
		if (fwrite(out_buf, 1, buf.out_pos, out) != buf.out_pos) {
			result = 5;
			break;
		}
		buf.out_pos = 0;
		if (ret == XZ_STREAM_END)
			break;
		if (ret != XZ_OK && ret != XZ_UNSUPPORTED_CHECK) {
			result = 10 + (int)ret;
			break;
		}
	}

	xz_dec_end(dec);
	fclose(in);
	if (fclose(out) != 0 && result == 0)
		result = 6;
	return result;
}
