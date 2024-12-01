package utils

import (
	"fmt"
	"time"
)

func FormatDuration(d time.Duration) string {
	if d < time.Microsecond {
		return fmt.Sprintf("%d ns", d.Nanoseconds())
	} else if d < time.Millisecond {
		return fmt.Sprintf("%.2f µs", float64(d.Microseconds())+float64(d.Nanoseconds()%1e3)/1e3)
	} else if d < time.Second {
		return fmt.Sprintf("%.2f ms", float64(d.Milliseconds())+float64(d.Microseconds()%1e3)/1e3)
	} else {
		return fmt.Sprintf("%.2f s", d.Seconds())
	}
}
