workspace "LoveAdmin Reconcilliation tool" {

    model {
        user = person "User" "Club admin"
        LoveAdminTool = softwareSystem "loveadmin_tool" "Love Admin Reconcilliation Tool" {
            !docs docs
            !adrs adrs

            LoveAdminTool-database = container "SQLite database"{
                tags "Database"
            }
            LoveAdminTool-database-if = container "db interface"
            LoveAdminTool-GUI = container "GUI Frontend"
            LoveAdminTool-xlsx_parser_LA = container "Parse LoveAadmin xlsx"
            LoveAdminTool-xlsx_parser_WG = container "Parse WholeGame xlsx"
            LoveAdminTool-db_query = container "Query db"
        }
        FA_Wholegame = softwareSystem "FA Wholegame"
        LoveAdmin = softwareSystem "LoveAdmin"

        LoveAdmin_xlsx = softwareSystem "LoveAdmin xlsx"{
            tags "file"
        }

        FA_Wholegame_xlsx = softwareSystem "FA_Wholegame xlsx"{
            tags "file"
        }
        
        DB_tranform_and_queries_json = softwareSystem "Config - Definition of data transform and queries"{
            tags "file"
        }
        
        user -> LoveAdminTool-GUI "Uses"
        user -> FA_Wholegame "Download xlsx"
        FA_Wholegame -> FA_Wholegame_xlsx "Save xlsx"
        user -> LoveAdmin "Download csv"
        LoveAdmin -> LoveAdmin_xlsx "Save xlsx"
        
        
        user -> LoveAdmin_xlsx "uploads xlsx"
        user -> FA_Wholegame_xlsx "uploads xlsx"
        LoveAdmin_xlsx ->  LoveAdminTool-xlsx_parser_LA  "LoveAdmin xlsx uploaded"
        FA_Wholegame_xlsx ->  LoveAdminTool-xlsx_parser_WG  "Wholegame xlsx uploaded"
        DB_tranform_and_queries_json -> LoveAdminTool-db_query "json loaded"
        
        
        LoveAdminTool-GUI -> LoveAdminTool-xlsx_parser_WG
        LoveAdminTool-GUI -> LoveAdminTool-xlsx_parser_LA
        LoveAdminTool-GUI -> LoveAdminTool-db_query
        LoveAdminTool-xlsx_parser_WG ->  LoveAdminTool-database-if
        LoveAdminTool-xlsx_parser_LA ->  LoveAdminTool-database-if
        LoveAdminTool-db_query ->  LoveAdminTool-database-if
        LoveAdminTool-database-if -> LoveAdminTool-database
    }

    views {
        systemLandscape LoveAdminTool_systemLandscape {
            include *
            autolayout
        }
    
        systemContext LoveAdminTool "loveadmin_tool" "SystemContext" {
            include * FA_Wholegame LoveAdmin
            autoLayout
        }
        
        container LoveAdminTool "Containers_All" {
            include *
            autolayout
        }
        
        dynamic LoveAdminTool "SimpleJourney" {
            title "Simple reconcile"
            autolayout
            user -> FA_Wholegame "Download xlsx info from FA Wholegame"
            user -> LoveAdmin "Download csv infor from LoveAdmin"
            user -> LoveAdminTool-GUI "load FA Wholegame xlsx into tool"
            user -> LoveAdminTool-GUI "load LoveAdmin csv into tool"
            user -> LoveAdminTool-GUI "Request Reconcilliation report"
        }
                
        dynamic LoveAdminTool "LoadFile" {
            title "Reconcile"
            autolayout
            user -> FA_Wholegame "Download xlsx info from FA Wholegame"
            FA_Wholegame -> FA_Wholegame_xlsx "file downloaded"
            user -> LoveAdmin "Download csv infor from LoveAdmin"
            LoveAdmin -> LoveAdmin_xlsx "file downloaded"
            user -> LoveAdminTool-GUI "select FA Wholegame xlsx into tool"
            
            LoveAdminTool-GUI -> LoveAdminTool-xlsx_parser_WG "Provide file name to parser"
            FA_Wholegame_xlsx  -> LoveAdminTool-xlsx_parser_WG "load file"
            LoveAdminTool-xlsx_parser_WG -> LoveAdminTool-database-if "sennd parsed data to db if"
            LoveAdminTool-database-if -> LoveAdminTool-database "store data in db"
            user -> LoveAdminTool-GUI "load LoveAdmin csv into tool"
            
            LoveAdminTool-GUI -> LoveAdminTool-xlsx_parser_LA "Provide file name to parser"
            LoveAdmin_xlsx  -> LoveAdminTool-xlsx_parser_LA "load file"            
            LoveAdminTool-xlsx_parser_LA -> LoveAdminTool-database-if "sennd parsed data to db if"
            LoveAdminTool-database-if -> LoveAdminTool-database "store data in db"
            
            user -> LoveAdminTool-GUI "Request Reconcilliation report"
            LoveAdminTool-GUI -> LoveAdminTool-db_query "Request query of data"
            LoveAdminTool-db_query -> LoveAdminTool-database-if "send query to db"
            LoveAdminTool-database-if -> LoveAdminTool-database "query data in db"
        }
        
        styles {
            element "Software System" {
                background #e1a6f5
                color #ffffff
            }
            element "Person" {
                shape person
                background #08427b
                color #ffffff
            }
            element "Database" {
                shape Cylinder
            }
            element "file" {
                shape Folder
                background #484848
            }
        }
    }

}
