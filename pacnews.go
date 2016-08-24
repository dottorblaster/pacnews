package main

import "os"
import "github.com/fatih/color"
import "github.com/SlyMarbo/rss"
import "github.com/kennygrant/sanitize"

func lpad(s string) string {
	return "  " + s
}

func print_news(news *rss.Item) {
	color.New(color.FgBlue, color.Bold).Println("\n" + lpad(news.Title))
	color.Blue(lpad(news.Link))
	color.Cyan(lpad(news.Date.String()[0:19]))
	color.White("\n" + sanitize.HTML(news.Content) + "\n\n")
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
