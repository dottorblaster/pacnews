package main

import "os"
import "github.com/fatih/color"
import "github.com/SlyMarbo/rss"

func print_news(news *rss.Item) {
	color.Yellow("---")
	color.Blue("  " + news.Title)
  color.White("  " + news.Date.String())
	color.Yellow("---\n")
}

func main() {
	feed, err := rss.Fetch("https://www.archlinux.org/feeds/news/")
	if err != nil {
		color.Red("Error fetching the Arch News RSS!\n")
		os.Exit(1)
	}

	for i := 0; i < len(feed.Items); i++ {
		print_news(feed.Items[i])
	}

	os.Exit(0)
}
