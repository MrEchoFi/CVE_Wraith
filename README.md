# Installation Process

[1]From the NATIONAL VULNERABILITY DATABASE (NVD),
You need to  generate your NVD API KEY from this -> https://nvd.nist.gov/developers/request-an-api-key

Now, when you will see that , NVD have been send the UUID in your provided mail then copy the UUID & go to the this link -> https://nvd.nist.gov/developers/confirm-api-key

[2] Now type your mail(The mail you provided to NVD) and UUID then, click 'Confirm'. Now you will see the 'API KEY'. So, copy the API KEY and STORE the 'NVD API KEY' in a safe file or any text file of yours.

[3] 

git clone <github link>

cd <working directory>

//in linux type:

export NVD_API_KEY="your generated api key via NVD"

//in windows powershell type,

setx NVD_API_KEY "your_api_key_here"

[Remember]- you have to maintain this 'export' methode when you open a new terminal also .

then type the generated api key in line- 459,

  .expect("type your NVD API KEY here"); 

# Now RUN:

//video

[~] USAGE:
 
| Key         | Action             |
| ----------- | ------------------ |
| Tab         | Switch input field |
| Enter       | Fetch CVEs         |
| ↑ ↓         | Scroll logs        |
| Mouse wheel | Scroll logs        |
| Esc         | Exit               |




# ## NOTE: In this project why NET_Wraith use Api key:

[[]] Without Api key:

[1] Limits:

- 5 requests per 30 seconds

- Stricter rate limiting

- More likely to receive HTTP 429

- Slower if paginating large result sets

[2] Good for:

- Small experiments

- Single version lookup

- Occasional manual tests

[3] Not good for:

- Scanner tools

- Automation

- CI pipelines

- Bulk CVE enumeration

[[~]] With NVD API KEY(Recommended):

[1] Benefits:

- Higher rate limits

- More stable throughput

- Reduced throttling

- Better for pagination

- Production-safe


[2]Performance

Raw latency per request is similar,
but total time is much lower when scanning multiple pages.

[Note] If you scan:

- 1 request → difference minimal

- 10–100+ requests → API key dramatically faster
