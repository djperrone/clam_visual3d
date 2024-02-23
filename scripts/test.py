import requests
import re
import tqdm
import os
import platform

# e.g. DROPBOX_URL ='https://www.dropbox.com/sh/23j4ldkj3jk32m/3lkjdlk3j2k34k4kkdkdjf?dl=0'
DROPBOX_URL = 'https://www.dropbox.com/s/n3wurjt8v9qi6nc/mnist.mat?dl=0'
DESTINATION_FOLDER = '.'
MAX_CONCURRENT_DOWNLOADS = 5


def buildurls(urls_raw, filetype='.JPG'):
    """ Create a list of download URLs - one for each file """

    # Filter list for duplicates - turn it into a set and then into a list again
    urls_raw = list(set(urls_raw))

    # Construct a list of tuples containing filename and download URL
    return [(url[:-5].split('/')[6], url.replace('dl=0', 'dl=1')) for url in urls_raw if filetype in url]


def writetofile(directory, filename, content):
    """ Helper method to write files onto the local filesystem """

    if not os.path.exists(directory):
        os.makedirs(directory)

    with open(os.path.join(directory, filename), 'wb') as f:
        f.write(content)
    print(f"File written to: {os.path.join(directory, filename)}")


def download(url, filename, directory):
    """ Download a file specified by URL """

    print(f"Downloading file from URL: {url}")
    response = requests.get(url)
    if response.status_code == 200:
        content = response.content
        writetofile(directory, filename, content)
    else:
        print(f"Failed to download file from {url}. Status code: {response.status_code}")


def parseresponse(url):
    """ Filter the response for all available items via href """

    print(f"Parsing response from URL: {url}")
    response = requests.get(url)
    urls = re.findall(r'href=[\'"]?([^\'" >]+)', response.text)
    print(f"Found URLs: {urls}")
    return urls


def main():
    """ Start the download of the given Dropbox directory """
    print(f"Operating system: {platform.system()}")

    # Parse the site for URLs and extract the filenames
    urls_raw = parseresponse(DROPBOX_URL)
    urls_download = buildurls(urls_raw)

    # Start the download manager
    print("Starting download manager...")
    for filename, url in tqdm.tqdm(urls_download):
        download(url, filename, DESTINATION_FOLDER)


if __name__ == '__main__':
    main()
