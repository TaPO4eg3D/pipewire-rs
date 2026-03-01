#include <spa/buffer/meta.h>

bool libspa_rs_meta_check(const void *p, const struct spa_meta *m) {
    return spa_meta_check(p, m);
}
