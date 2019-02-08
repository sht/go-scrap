package main

import (
	"image/png"
	"log"
	"os"
	"time"

	"github.com/cretz/go-scrap"
)

func main() {
	fileName := "temp.png"
	if len(os.Args) > 1 {
		fileName = os.Args[1]
	}
	if err := saveScreenshotPNG(fileName); err != nil {
		log.Fatalf("Failed creating screenshot at %v: %v", fileName, err)
	}
	log.Printf("Created screenshot at %v", fileName)
}

func saveScreenshotPNG(fileName string) error {
	img, err := getScreenshot()
	if err != nil {
		return err
	}
	file, err := os.Create(fileName)
	if err != nil {
		return err
	}
	defer file.Close()
	return png.Encode(file, img)
}

func getScreenshot() (*scrap.FrameImage, error) {
	// Get the main display
	d, err := scrap.PrimaryDisplay()
	if err != nil {
		return nil, err
	}
	// Create capturer for it
	c, err := scrap.NewCapturer(d)
	if err != nil {
		return nil, err
	}
	// Get an image, trying until one available
	for {
		if img, _, err := c.FrameImage(); img != nil || err != nil {
			return img, err
		}
		// Sleep 17ms (~1/60th of a second)
		time.Sleep(17 * time.Millisecond)
	}
}