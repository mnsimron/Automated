<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_Content Management System             _4a83bb</name>
   <tag></tag>
   <elementGuidId>7e0d6b2b-9fbf-45aa-b1ae-3b691e5c0eb8</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>body.login</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>0106d164-6e6e-4107-9538-27ef2bea87d8</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>login</value>
      <webElementGuid>6d944245-bc0a-4ec9-8d44-30d7fc244c4a</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    
    
    
        
        Content Management System
    
    
    
        
            
                
                
                    Enter username
                
            
            
                Username
                
            
            
                Password
                
            
            
                Login
            
        
        
            Forget Password ?
            
                Enter your e-mail address below to reset your password.
            
            
                
            
            
                Back
                Submit
            
        
    
    
        2019 © CMS APL All Rights Reserved
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
        jQuery(document).ready(function () {
            $.validator.addMethod(&quot;noSpace&quot;, function (value, element) {
                return value.indexOf(&quot; &quot;) &lt; 0 &amp;&amp; value != &quot;&quot;;
            }, &quot;No space please and don't leave it empty&quot;);
            Metronic.init(); // init metronic core components
            Layout.init(); // init current layout
            //Login.init();
            //Demo.init();
            ; (function ($) {

                'use strict';

                $('.alert[data-auto-dismiss]').each(function (index, element) {
                    var $element = $(element),
                        timeout = $element.data('auto-dismiss') || 2000;

                    setTimeout(function () {
                        $element.slideUp(500);
                    }, timeout);
                });


                $(&quot;#FormLogin&quot;).validate({
                    errorElement: &quot;em&quot;,
                    rules: {
                        username: {
                            required: true
                        },
                        password: {
                            noSpace: true,
                            required: true
                        }
                    },
                    messages: {
                        username: {
                            required: &quot;Username Required.&quot;,
                        },
                        password: {
                            required: &quot;Password Required.&quot;,
                            noSpace : &quot;Space not allowed.&quot;
                        },
                    }
                });
                $(&quot;#btnLogin&quot;).click(function (e) {
                    e.preventDefault();
                    if ($(&quot;#FormLogin&quot;).valid()) {
                        $(&quot;#FormLogin&quot;).submit();
                    }
                });
                $('#username').keydown(function (e) {
                    if (e.keyCode == 13) {
                        if ($(&quot;#FormLogin&quot;).valid()) {
                            $(&quot;#FormLogin&quot;).submit();
                        }
                    }
                })

                $('#password').keydown(function (e) {
                    console.log(e.keyCode)
                    if (e.keyCode == 13) {
                        if ($(&quot;#FormLogin&quot;).valid()) {
                            $(&quot;#FormLogin&quot;).submit();
                        }
                    }
                })
            })(jQuery);
        });
    

/html[1]/body[@class=&quot;login&quot;]</value>
      <webElementGuid>abe2ea05-60fe-4ce5-bcea-5386d9e3afdf</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[@class=&quot;login&quot;]</value>
      <webElementGuid>f545a9cb-9393-463a-9183-c7cb7b86453e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>2f30ad67-ae64-4293-9606-420794984ec6</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>login</value>
      <webElementGuid>ce888d02-c9e8-489c-a16b-2e39ac7f6226</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    
    
    
        
        Content Management System
    
    
    
        
            
                
                
                    Enter username
                
            
            
                Username
                
            
            
                Password
                
            
            
                Login
            
        
        
            Forget Password ?
            
                Enter your e-mail address below to reset your password.
            
            
                
            
            
                Back
                Submit
            
        
    
    
        2019 © CMS APL All Rights Reserved
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
        jQuery(document).ready(function () {
            $.validator.addMethod(&quot;noSpace&quot;, function (value, element) {
                return value.indexOf(&quot; &quot;) &lt; 0 &amp;&amp; value != &quot;&quot;;
            }, &quot;No space please and don't leave it empty&quot;);
            Metronic.init(); // init metronic core components
            Layout.init(); // init current layout
            //Login.init();
            //Demo.init();
            ; (function ($) {

                'use strict';

                $('.alert[data-auto-dismiss]').each(function (index, element) {
                    var $element = $(element),
                        timeout = $element.data('auto-dismiss') || 2000;

                    setTimeout(function () {
                        $element.slideUp(500);
                    }, timeout);
                });


                $(&quot;#FormLogin&quot;).validate({
                    errorElement: &quot;em&quot;,
                    rules: {
                        username: {
                            required: true
                        },
                        password: {
                            noSpace: true,
                            required: true
                        }
                    },
                    messages: {
                        username: {
                            required: &quot;Username Required.&quot;,
                        },
                        password: {
                            required: &quot;Password Required.&quot;,
                            noSpace : &quot;Space not allowed.&quot;
                        },
                    }
                });
                $(&quot;#btnLogin&quot;).click(function (e) {
                    e.preventDefault();
                    if ($(&quot;#FormLogin&quot;).valid()) {
                        $(&quot;#FormLogin&quot;).submit();
                    }
                });
                $('#username').keydown(function (e) {
                    if (e.keyCode == 13) {
                        if ($(&quot;#FormLogin&quot;).valid()) {
                            $(&quot;#FormLogin&quot;).submit();
                        }
                    }
                })

                $('#password').keydown(function (e) {
                    console.log(e.keyCode)
                    if (e.keyCode == 13) {
                        if ($(&quot;#FormLogin&quot;).valid()) {
                            $(&quot;#FormLogin&quot;).submit();
                        }
                    }
                })
            })(jQuery);
        });
    

/html[1]/body[@class=&quot;login&quot;]</value>
      <webElementGuid>49aab5fe-a53d-46a8-ae4f-573e8209163b</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[@class=&quot;login&quot;]</value>
      <webElementGuid>a5603c6a-cf5b-4d74-8cd6-05b86dea5789</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>6a4a0ffa-f74b-41e3-b60a-bf42d80547e7</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>login</value>
      <webElementGuid>5e1e2796-7f5d-46bb-8268-722de4a81a23</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    
    
    
        
        Content Management System
    
    
    
        
            
                
                
                    Enter username
                
            
            
                Username
                
            
            
                Password
                
            
            
                Login
            
        
        
            Forget Password ?
            
                Enter your e-mail address below to reset your password.
            
            
                
            
            
                Back
                Submit
            
        
    
    
        2019 © CMS APL All Rights Reserved
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
        jQuery(document).ready(function () {
            $.validator.addMethod(&quot;noSpace&quot;, function (value, element) {
                return value.indexOf(&quot; &quot;) &lt; 0 &amp;&amp; value != &quot;&quot;;
            }, &quot;No space please and don't leave it empty&quot;);
            Metronic.init(); // init metronic core components
            Layout.init(); // init current layout
            //Login.init();
            //Demo.init();
            ; (function ($) {

                'use strict';

                $('.alert[data-auto-dismiss]').each(function (index, element) {
                    var $element = $(element),
                        timeout = $element.data('auto-dismiss') || 2000;

                    setTimeout(function () {
                        $element.slideUp(500);
                    }, timeout);
                });


                $(&quot;#FormLogin&quot;).validate({
                    errorElement: &quot;em&quot;,
                    rules: {
                        username: {
                            required: true
                        },
                        password: {
                            noSpace: true,
                            required: true
                        }
                    },
                    messages: {
                        username: {
                            required: &quot;Username Required.&quot;,
                        },
                        password: {
                            required: &quot;Password Required.&quot;,
                            noSpace : &quot;Space not allowed.&quot;
                        },
                    }
                });
                $(&quot;#btnLogin&quot;).click(function (e) {
                    e.preventDefault();
                    if ($(&quot;#FormLogin&quot;).valid()) {
                        $(&quot;#FormLogin&quot;).submit();
                    }
                });
                $('#username').keydown(function (e) {
                    if (e.keyCode == 13) {
                        if ($(&quot;#FormLogin&quot;).valid()) {
                            $(&quot;#FormLogin&quot;).submit();
                        }
                    }
                })

                $('#password').keydown(function (e) {
                    console.log(e.keyCode)
                    if (e.keyCode == 13) {
                        if ($(&quot;#FormLogin&quot;).valid()) {
                            $(&quot;#FormLogin&quot;).submit();
                        }
                    }
                })
            })(jQuery);
        });
    

/html[1]/body[@class=&quot;login&quot;]</value>
      <webElementGuid>2177e773-5b81-455b-94f7-456967f9452f</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[@class=&quot;login&quot;]</value>
      <webElementGuid>81ae8548-edaa-415c-a04c-980a885abfc0</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>68272f38-c41c-4fd6-8a06-6c290e0911d4</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>login</value>
      <webElementGuid>f0a2cb95-da88-4aad-8a83-e97ce5172fc6</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    
    
    
        
        Content Management System
    
    
    
        
            
                
                
                    Enter username
                
            
            
                Username
                
            
            
                Password
                
            
            
                Login
            
        
        
            Forget Password ?
            
                Enter your e-mail address below to reset your password.
            
            
                
            
            
                Back
                Submit
            
        
    
    
        2019 © CMS APL All Rights Reserved
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
        jQuery(document).ready(function () {
            $.validator.addMethod(&quot;noSpace&quot;, function (value, element) {
                return value.indexOf(&quot; &quot;) &lt; 0 &amp;&amp; value != &quot;&quot;;
            }, &quot;No space please and don't leave it empty&quot;);
            Metronic.init(); // init metronic core components
            Layout.init(); // init current layout
            //Login.init();
            //Demo.init();
            ; (function ($) {

                'use strict';

                $('.alert[data-auto-dismiss]').each(function (index, element) {
                    var $element = $(element),
                        timeout = $element.data('auto-dismiss') || 2000;

                    setTimeout(function () {
                        $element.slideUp(500);
                    }, timeout);
                });


                $(&quot;#FormLogin&quot;).validate({
                    errorElement: &quot;em&quot;,
                    rules: {
                        username: {
                            required: true
                        },
                        password: {
                            noSpace: true,
                            required: true
                        }
                    },
                    messages: {
                        username: {
                            required: &quot;Username Required.&quot;,
                        },
                        password: {
                            required: &quot;Password Required.&quot;,
                            noSpace : &quot;Space not allowed.&quot;
                        },
                    }
                });
                $(&quot;#btnLogin&quot;).click(function (e) {
                    e.preventDefault();
                    if ($(&quot;#FormLogin&quot;).valid()) {
                        $(&quot;#FormLogin&quot;).submit();
                    }
                });
                $('#username').keydown(function (e) {
                    if (e.keyCode == 13) {
                        if ($(&quot;#FormLogin&quot;).valid()) {
                            $(&quot;#FormLogin&quot;).submit();
                        }
                    }
                })

                $('#password').keydown(function (e) {
                    console.log(e.keyCode)
                    if (e.keyCode == 13) {
                        if ($(&quot;#FormLogin&quot;).valid()) {
                            $(&quot;#FormLogin&quot;).submit();
                        }
                    }
                })
            })(jQuery);
        });
    

id(&quot;username&quot;)</value>
      <webElementGuid>cd8b067d-689b-4cd4-961c-5081dade78b4</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[@class=&quot;login&quot;]</value>
      <webElementGuid>65645098-6282-4631-a0f3-35fc83865097</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>cf99816e-c278-46ae-931d-ff99aeb24106</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;
    
    
    
        
        Content Management System
    
    
    
        
            
                
                
                    Enter username
                
            
            
                Username
                
            
            
                Password
                
            
            
                Login
            
        
        
            Forget Password ?
            
                Enter your e-mail address below to reset your password.
            
            
                
            
            
                Back
                Submit
            
        
    
    
        2019 © CMS APL All Rights Reserved
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
        jQuery(document).ready(function () {
            $.validator.addMethod(&quot;noSpace&quot;, function (value, element) {
                return value.indexOf(&quot; &quot;) &lt; 0 &amp;&amp; value != &quot;&quot;;
            }, &quot;No space please and don&quot; , &quot;'&quot; , &quot;t leave it empty&quot;);
            Metronic.init(); // init metronic core components
            Layout.init(); // init current layout
            //Login.init();
            //Demo.init();
            ; (function ($) {

                &quot; , &quot;'&quot; , &quot;use strict&quot; , &quot;'&quot; , &quot;;

                $(&quot; , &quot;'&quot; , &quot;.alert[data-auto-dismiss]&quot; , &quot;'&quot; , &quot;).each(function (index, element) {
                    var $element = $(element),
                        timeout = $element.data(&quot; , &quot;'&quot; , &quot;auto-dismiss&quot; , &quot;'&quot; , &quot;) || 2000;

                    setTimeout(function () {
                        $element.slideUp(500);
                    }, timeout);
                });


                $(&quot;#FormLogin&quot;).validate({
                    errorElement: &quot;em&quot;,
                    rules: {
                        username: {
                            required: true
                        },
                        password: {
                            noSpace: true,
                            required: true
                        }
                    },
                    messages: {
                        username: {
                            required: &quot;Username Required.&quot;,
                        },
                        password: {
                            required: &quot;Password Required.&quot;,
                            noSpace : &quot;Space not allowed.&quot;
                        },
                    }
                });
                $(&quot;#btnLogin&quot;).click(function (e) {
                    e.preventDefault();
                    if ($(&quot;#FormLogin&quot;).valid()) {
                        $(&quot;#FormLogin&quot;).submit();
                    }
                });
                $(&quot; , &quot;'&quot; , &quot;#username&quot; , &quot;'&quot; , &quot;).keydown(function (e) {
                    if (e.keyCode == 13) {
                        if ($(&quot;#FormLogin&quot;).valid()) {
                            $(&quot;#FormLogin&quot;).submit();
                        }
                    }
                })

                $(&quot; , &quot;'&quot; , &quot;#password&quot; , &quot;'&quot; , &quot;).keydown(function (e) {
                    console.log(e.keyCode)
                    if (e.keyCode == 13) {
                        if ($(&quot;#FormLogin&quot;).valid()) {
                            $(&quot;#FormLogin&quot;).submit();
                        }
                    }
                })
            })(jQuery);
        });
    

/html[1]/body[@class=&quot;login&quot;]&quot;) or . = concat(&quot;
    
    
    
        
        Content Management System
    
    
    
        
            
                
                
                    Enter username
                
            
            
                Username
                
            
            
                Password
                
            
            
                Login
            
        
        
            Forget Password ?
            
                Enter your e-mail address below to reset your password.
            
            
                
            
            
                Back
                Submit
            
        
    
    
        2019 © CMS APL All Rights Reserved
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
        jQuery(document).ready(function () {
            $.validator.addMethod(&quot;noSpace&quot;, function (value, element) {
                return value.indexOf(&quot; &quot;) &lt; 0 &amp;&amp; value != &quot;&quot;;
            }, &quot;No space please and don&quot; , &quot;'&quot; , &quot;t leave it empty&quot;);
            Metronic.init(); // init metronic core components
            Layout.init(); // init current layout
            //Login.init();
            //Demo.init();
            ; (function ($) {

                &quot; , &quot;'&quot; , &quot;use strict&quot; , &quot;'&quot; , &quot;;

                $(&quot; , &quot;'&quot; , &quot;.alert[data-auto-dismiss]&quot; , &quot;'&quot; , &quot;).each(function (index, element) {
                    var $element = $(element),
                        timeout = $element.data(&quot; , &quot;'&quot; , &quot;auto-dismiss&quot; , &quot;'&quot; , &quot;) || 2000;

                    setTimeout(function () {
                        $element.slideUp(500);
                    }, timeout);
                });


                $(&quot;#FormLogin&quot;).validate({
                    errorElement: &quot;em&quot;,
                    rules: {
                        username: {
                            required: true
                        },
                        password: {
                            noSpace: true,
                            required: true
                        }
                    },
                    messages: {
                        username: {
                            required: &quot;Username Required.&quot;,
                        },
                        password: {
                            required: &quot;Password Required.&quot;,
                            noSpace : &quot;Space not allowed.&quot;
                        },
                    }
                });
                $(&quot;#btnLogin&quot;).click(function (e) {
                    e.preventDefault();
                    if ($(&quot;#FormLogin&quot;).valid()) {
                        $(&quot;#FormLogin&quot;).submit();
                    }
                });
                $(&quot; , &quot;'&quot; , &quot;#username&quot; , &quot;'&quot; , &quot;).keydown(function (e) {
                    if (e.keyCode == 13) {
                        if ($(&quot;#FormLogin&quot;).valid()) {
                            $(&quot;#FormLogin&quot;).submit();
                        }
                    }
                })

                $(&quot; , &quot;'&quot; , &quot;#password&quot; , &quot;'&quot; , &quot;).keydown(function (e) {
                    console.log(e.keyCode)
                    if (e.keyCode == 13) {
                        if ($(&quot;#FormLogin&quot;).valid()) {
                            $(&quot;#FormLogin&quot;).submit();
                        }
                    }
                })
            })(jQuery);
        });
    

/html[1]/body[@class=&quot;login&quot;]&quot;))]</value>
      <webElementGuid>9c52d859-3224-4930-bcca-1989c485d90f</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;
    
    
    
        
        Content Management System
    
    
    
        
            
                
                
                    Enter username
                
            
            
                Username
                
            
            
                Password
                
            
            
                Login
            
        
        
            Forget Password ?
            
                Enter your e-mail address below to reset your password.
            
            
                
            
            
                Back
                Submit
            
        
    
    
        2019 © CMS APL All Rights Reserved
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
        jQuery(document).ready(function () {
            $.validator.addMethod(&quot;noSpace&quot;, function (value, element) {
                return value.indexOf(&quot; &quot;) &lt; 0 &amp;&amp; value != &quot;&quot;;
            }, &quot;No space please and don&quot; , &quot;'&quot; , &quot;t leave it empty&quot;);
            Metronic.init(); // init metronic core components
            Layout.init(); // init current layout
            //Login.init();
            //Demo.init();
            ; (function ($) {

                &quot; , &quot;'&quot; , &quot;use strict&quot; , &quot;'&quot; , &quot;;

                $(&quot; , &quot;'&quot; , &quot;.alert[data-auto-dismiss]&quot; , &quot;'&quot; , &quot;).each(function (index, element) {
                    var $element = $(element),
                        timeout = $element.data(&quot; , &quot;'&quot; , &quot;auto-dismiss&quot; , &quot;'&quot; , &quot;) || 2000;

                    setTimeout(function () {
                        $element.slideUp(500);
                    }, timeout);
                });


                $(&quot;#FormLogin&quot;).validate({
                    errorElement: &quot;em&quot;,
                    rules: {
                        username: {
                            required: true
                        },
                        password: {
                            noSpace: true,
                            required: true
                        }
                    },
                    messages: {
                        username: {
                            required: &quot;Username Required.&quot;,
                        },
                        password: {
                            required: &quot;Password Required.&quot;,
                            noSpace : &quot;Space not allowed.&quot;
                        },
                    }
                });
                $(&quot;#btnLogin&quot;).click(function (e) {
                    e.preventDefault();
                    if ($(&quot;#FormLogin&quot;).valid()) {
                        $(&quot;#FormLogin&quot;).submit();
                    }
                });
                $(&quot; , &quot;'&quot; , &quot;#username&quot; , &quot;'&quot; , &quot;).keydown(function (e) {
                    if (e.keyCode == 13) {
                        if ($(&quot;#FormLogin&quot;).valid()) {
                            $(&quot;#FormLogin&quot;).submit();
                        }
                    }
                })

                $(&quot; , &quot;'&quot; , &quot;#password&quot; , &quot;'&quot; , &quot;).keydown(function (e) {
                    console.log(e.keyCode)
                    if (e.keyCode == 13) {
                        if ($(&quot;#FormLogin&quot;).valid()) {
                            $(&quot;#FormLogin&quot;).submit();
                        }
                    }
                })
            })(jQuery);
        });
    

id(&quot;username&quot;)&quot;) or . = concat(&quot;
    
    
    
        
        Content Management System
    
    
    
        
            
                
                
                    Enter username
                
            
            
                Username
                
            
            
                Password
                
            
            
                Login
            
        
        
            Forget Password ?
            
                Enter your e-mail address below to reset your password.
            
            
                
            
            
                Back
                Submit
            
        
    
    
        2019 © CMS APL All Rights Reserved
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
        jQuery(document).ready(function () {
            $.validator.addMethod(&quot;noSpace&quot;, function (value, element) {
                return value.indexOf(&quot; &quot;) &lt; 0 &amp;&amp; value != &quot;&quot;;
            }, &quot;No space please and don&quot; , &quot;'&quot; , &quot;t leave it empty&quot;);
            Metronic.init(); // init metronic core components
            Layout.init(); // init current layout
            //Login.init();
            //Demo.init();
            ; (function ($) {

                &quot; , &quot;'&quot; , &quot;use strict&quot; , &quot;'&quot; , &quot;;

                $(&quot; , &quot;'&quot; , &quot;.alert[data-auto-dismiss]&quot; , &quot;'&quot; , &quot;).each(function (index, element) {
                    var $element = $(element),
                        timeout = $element.data(&quot; , &quot;'&quot; , &quot;auto-dismiss&quot; , &quot;'&quot; , &quot;) || 2000;

                    setTimeout(function () {
                        $element.slideUp(500);
                    }, timeout);
                });


                $(&quot;#FormLogin&quot;).validate({
                    errorElement: &quot;em&quot;,
                    rules: {
                        username: {
                            required: true
                        },
                        password: {
                            noSpace: true,
                            required: true
                        }
                    },
                    messages: {
                        username: {
                            required: &quot;Username Required.&quot;,
                        },
                        password: {
                            required: &quot;Password Required.&quot;,
                            noSpace : &quot;Space not allowed.&quot;
                        },
                    }
                });
                $(&quot;#btnLogin&quot;).click(function (e) {
                    e.preventDefault();
                    if ($(&quot;#FormLogin&quot;).valid()) {
                        $(&quot;#FormLogin&quot;).submit();
                    }
                });
                $(&quot; , &quot;'&quot; , &quot;#username&quot; , &quot;'&quot; , &quot;).keydown(function (e) {
                    if (e.keyCode == 13) {
                        if ($(&quot;#FormLogin&quot;).valid()) {
                            $(&quot;#FormLogin&quot;).submit();
                        }
                    }
                })

                $(&quot; , &quot;'&quot; , &quot;#password&quot; , &quot;'&quot; , &quot;).keydown(function (e) {
                    console.log(e.keyCode)
                    if (e.keyCode == 13) {
                        if ($(&quot;#FormLogin&quot;).valid()) {
                            $(&quot;#FormLogin&quot;).submit();
                        }
                    }
                })
            })(jQuery);
        });
    

id(&quot;username&quot;)&quot;))]</value>
      <webElementGuid>113c3202-9ea6-4f75-928d-30d3a9ac6e24</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
