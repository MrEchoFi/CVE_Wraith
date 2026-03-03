<pre>_____________   _______________   __      __               .__  __     ___ ___  
\_   ___ \   \ /   /\_   _____/  /  \    /  \____________  |__|/  |_  /   |   \ 
/    \  \/\   Y   /  |    __)_   \   \/\/   /\_  __ \__  \ |  \   __\/    ~    \
\     \____\     /   |        \   \        /  |  | \// __ \|  ||  |  \    Y    /
 \______  / \___/   /_______  /____\__/\  /   |__|  (____  /__||__|   \___|_  / 
        \/                  \/_____/    \/               \/                 \/  </pre>


</pre>
<div align="center">


  <img src="https://github.com/MrEchoFi/MrEchoFi/raw/4274f537dec313ac7dde4403fe0fae24259beade/Mr.EchoFi-New-Logo-with-ASCII.jpg" alt="logo" width="240" height="auto" />
  <h1>CVE Wraith</h1>
   
  <p>
   Find any CVE via Vendor name-Product name-Version
  </p>


  📫 How to reach me  **[Website](https://echo-fi-portfolio-node-js.vercel.app/)** ,  **[Website 2](https://tanjib-portfolio-website.vercel.app/)**, **http://mrechofi.github.io/Tanjib_portfolio_website/** & **tanjibisham777@gmail.com & tanjibisham888@gmail.com**

</div>

# Installation Process

- [1]From the NATIONAL VULNERABILITY DATABASE (NVD),
You need to  generate your NVD API KEY from this -> https://nvd.nist.gov/developers/request-an-api-key

Now, when you will see that , NVD have been send the UUID in your provided mail then copy the UUID & go to the this link -> https://nvd.nist.gov/developers/confirm-api-key

- [2] Now type your mail(The mail you provided to NVD) and UUID then, click 'Confirm'. Now you will see the 'API KEY'. So, copy the API KEY and STORE the 'NVD API KEY' in a safe file or any text file of yours.
-  [Remember]- you have to maintain this 'export' methode when you open a new terminal also . Then type the generated api key in line-'459',


       .expect("type your NVD API KEY here"); 

- [3] Using < git clone > :


      git clone https://github.com/MrEchoFi/CVE_Wraith.git

      cd CVE_Wraith

      //in linux type:

      export NVD_API_KEY="your generated api key via NVD"

      //in windows powershell type,

      setx NVD_API_KEY "your_api_key_here"

      cargo run

  - OR > Open 'run.sh' file & navigate the 'NVD API KEY' to this  [export NVD_API_KEY="your generated api key via NVD"] ; Now run:
  
            chmod +x run.sh

            ./run.sh

            NOTE: Via this "./run.sh " method ; you do not need to type -> [export NVD_API_KEY="your generated api key via NVD"] in terminal while swithcing to new terminal or any terminal!! So, this is much more easy for you.
     
  




- [4] Install & run from SOURCE:
           
            git clone https://github.com/MrEchoFi/CVE_Wraith.git
            cd CVE_Wraith

            // In linux,
            export NVD_API_KEY="your generated api key via NVD"

            // In windows,
            setx NVD_API_KEY "your_api_key_here"

          
            cargo build --release
            ./target/release/<binary-name>

- OR > Open 'run1.sh' file & navigate the 'NVD API KEY' to this  [export NVD_API_KEY="your generated api key via NVD"] ; Now run:

            chmod +x run1.sh

            ./run1.sh

            NOTE: Via this "./run1.sh " method ; you do not need to type -> [export NVD_API_KEY="your generated api key via NVD"] in terminal while swithcing to new terminal or any terminal!! So, this is much more easy for you.

- [5] Install & run via 'cargo' :

       cargo install cve_wraith

       // In linux,
            export NVD_API_KEY="your generated api key via NVD"

      // In windows,
           setx NVD_API_KEY "your_api_key_here"

        Now run,

        cve_wraith

- OR > Open 'run2.sh' file & navigate the 'NVD API KEY' to this  [export NVD_API_KEY="your generated api key via NVD"] ; Now run:
  
            chmod +x run2.sh

            ./run2.sh

            NOTE: Via this "./run2.sh " method ; you do not need to type -> [export NVD_API_KEY="your generated api key via NVD"] in terminal while swithcing to new terminal or any terminal!! So, this is much more easy for you.
           



# video:

[cve_wraith2.webm](https://github.com/user-attachments/assets/3ec9768d-9548-45b8-be48-26704afc2a04)


[~] USAGE:
 
| Key         | Action             |
| ----------- | ------------------ |
| Tab         | Switch input field |
| Enter       | Fetch CVEs         |
| ↑ ↓         | Scroll logs        |
| Mouse wheel | Scroll logs        |
| Esc         | Exit               |




# ## NOTE: Why CVE_Wraith use Api key:

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
