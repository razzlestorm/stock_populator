use serde::Serialize;
use std::error::Error;
use std::thread;
use std::time::Duration;
use thirtyfour::prelude::*;

use url::Url;






#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new("http://localhost:4444", caps).await?;

    // check date and navigate to website 
    driver.goto("https://www.nasdaq.com/market-activity/earnings").await?;




    let elem_form = driver.find(By::ClassName("time-belt__list")).await?;


    // Find date elements that fall within the weekdays of the current date range
    let elem_button = elem_form.find(By::Css("button[class='time-belt__item']")).await?;
    elem_button.click().await?;

    // Look for header to implicitly wait for the page to load.
    driver.find(By::ClassName("firstHeading")).await?;
    assert_eq!(driver.title().await?, "Earnings Calendar | Nasdaq");

    // explicitly close the browser.
    driver.quit().await?;

    Ok(())
}
